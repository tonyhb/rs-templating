#![macro_use]

use tera::ast::{ExprVal, Node};
use wasm_bindgen::prelude::*;

const TEMPLATE_NAME: &str = "tpl";
const LOOP_CONSTS: [&str; 4] = ["loop.index", "loop.index0", "loop.first", "loop.last"];

// define a quick macro for converting a Result<T, <Box dyn std::error::Error>> into
// Result<T, JsValue>.
macro_rules! jserr {
    ($expression:expr) => {
        match $expression {
            Ok(a) => Ok(a),
            Err(e) => {
                return Err(JsValue::from(format!("{}", e)));
            }
        }
    };
}

#[wasm_bindgen]
pub struct Template {
    source: String,
    tera: tera::Tera,
}

#[wasm_bindgen]
impl Template {

    #[wasm_bindgen]
    pub fn new(source: String) -> Result<Template, JsValue> {
        return jserr!(Template::init(source))
    }

    #[wasm_bindgen]
    pub fn execute(&self, val: JsValue) -> Result<String, JsValue> {
        let context = match self.generate_context(val.as_string().ok_or("value is not a json encoded string")?) {
            Err(e) => return Err(JsValue::from(format!("error generating context: {}", e))),
            Ok(c) => c,
        };

        jserr!(self.execute_with_context(&context))
    }

    #[wasm_bindgen(getter)]
    pub fn variables(&self) -> Vec<JsValue> {
        let mut res: Vec<JsValue> = vec![];
        for str in self.get_variables().iter() {
            res.push(str.into());
        }
        return res
    }

    #[wasm_bindgen(getter)]
    pub fn source(&mut self) -> String {
        self.source.clone()
    }
}

// shared, non-wasm functions
impl Template {
    pub fn init(source: String) -> Result<Template, Box<dyn std::error::Error>> {
        let mut tpl = Template {
            source,
            tera: tera::Tera::default(),
        };
        tpl.tera.ignore_undefined = true;
        // Ensure we autoescape our template
        tpl.tera.autoescape_on(vec![TEMPLATE_NAME]);
        if let Err(val) = tpl.parse() {
            return Err(val);
        }
        return Ok(tpl);
    }

    // variables inspects the ast of the template to determine which variables are specified
    // in the template.
    // 
    // in short, the approach is to recursively iterate through the AST (eg. within loops,
    // if expressions, blocks) and determine any "{{ ident }}" nodes, then grab the identifier
    // specified.
    //
    // in practice, there are several nuances:
    //   - loop.{index, index0, first, last} shouldn't be counded within loops.
    //   - if a template contains a for loop ("{% for item in products %}"), item within
    //     the block shouldn't be counted.
    //
    pub fn get_variables(&self) -> Vec<String> {
        let tpl = self.tera.get_template(&"tpl").unwrap();

        // eval_expr evaluates the expression given to see whether we have
        // an identifier - a varaible that we should add.
        //
        // denylist represents any new variable created within tera;  these are not user defined
        // variables that we need to match against
        fn eval_expr(expr: &ExprVal, results: &mut Vec<String>, denylist: &mut Vec<String>) -> () {
            match expr {
                ExprVal::Ident(ref ident) => {
                    // There are several consts in Tera that shouldn't be counted here.
                    if LOOP_CONSTS.contains(&&*ident.to_string()) {
                        return;
                    }

                    if denylist.len() > 0 {
                        for item in denylist.iter() {
                            if item == ident {
                                return;
                            }
                            let dot_prefix = &format!("{}.", item);
                            let bracket_prefix = &format!("{}[", item);
                            if ident.starts_with(dot_prefix) || ident.starts_with(bracket_prefix) {
                                return;
                            }
                        }
                    }

                    if !results.contains(ident) {
                        results.push(ident.into());
                    }
                }
                ExprVal::StringConcat(ref concat) => {
                    // We may be concatenating two independent variables.
                    for val in &concat.values {
                        eval_expr(val, results, denylist);
                    }
                }
                ExprVal::Array(ref arr) => {
                    for expr in arr {
                        eval_expr(&expr.val, results, denylist);
                    }
                }
                ExprVal::MacroCall(call) => {
                    for (_arg_name, expr) in &call.args {
                        eval_expr(&expr.val, results, denylist);
                    }
                }
                ExprVal::FunctionCall(call) => {
                    for (_arg_name, expr) in &call.args {
                        eval_expr(&expr.val, results, denylist);
                    }
                }
                ExprVal::Logic(l) => {
                    eval_expr(&l.lhs.val, results, denylist);
                    eval_expr(&l.rhs.val, results, denylist);
                }
                ExprVal::Math(m) => {
                    eval_expr(&m.lhs.val, results, denylist);
                    eval_expr(&m.rhs.val, results, denylist);
                }
                _ => {}
            }
            ()
        }

        // iter_nodes iterates through each node in the ASt recursively, calling
        // eval_expr on expressions contained within the node to find variable names
        // within the template.
        fn iter_nodes(ast: &[Node], results: &mut Vec<String>, denylist: &mut Vec<String>) -> () {
            for node in ast.iter() {
                match node {
                    Node::Block(_, block, _) => {
                        iter_nodes(&block.body, results, denylist);
                    }
                    Node::VariableBlock(_, expr) => {
                        // a {{ }} node containing variables.
                        eval_expr(&expr.val, results, denylist);
                    }
                    Node::If(iff, _) => {
                        for condition in &iff.conditions {
                            eval_expr(&condition.1.val, results, denylist);
                            iter_nodes(&condition.2, results, denylist);
                        }
                        if let Some(otherwise) = &iff.otherwise {
                            iter_nodes(&otherwise.1, results, denylist);
                        }
                    }
                    Node::FilterSection(_, f, _) => {
                        for (_arg, expr) in &f.filter.args {
                            eval_expr(&expr.val, results, denylist);
                        }
                        iter_nodes(&f.body, results, denylist);
                    }
                    Node::Forloop(_, forr, _) => {
                        // Put the variable created within the for loop into the denylist.  Clone
                        // the list such that we don't mutate the shared reference and this lasts
                        // only for the recused calls.
                        let mut copy = denylist.clone();
                        copy.push(forr.value.clone());
                        // The "container" is the variable name being iterated over.
                        eval_expr(&forr.container.val, results, &mut copy);
                        iter_nodes(&forr.body, results, &mut copy);
                    }
                    Node::Set(_, set) => {
                        // When setting variables we want them included in the blacklist
                        // forevermore, so mutate the original denylist.
                        denylist.push(set.key.clone());
                        eval_expr(&set.value.val, results, denylist);
                    }
                    _ => {}
                }
                continue;
            }
            ()
        }

        let mut res: Vec<String> = vec![];
        let mut deny: Vec<String> = vec![];
        iter_nodes(&tpl.ast, &mut res, &mut deny);
        res
    }

    pub fn execute_with_context(&self, ctx: &tera::Context) -> Result<String, tera::Error> {
        self.tera.render("tpl", &ctx)
    }

    // generate_context creates templating context from a JSON-stringified object.
    fn generate_context(&self, val: String) -> Result<tera::Context, Box<dyn std::error::Error>> {
        let map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_str(&val)?;
        let ctx = tera::Context::from_serialize(map)?;
        Ok(ctx)
    }

    // parse parses the template using tera
    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.tera.add_raw_template(TEMPLATE_NAME, self.source.as_str())?;
        self.validate()?;
        Ok(())
    }

    fn validate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Inspect AST.  If the template uses
        //   - Node::Extends
        //   - Node::Include
        //   - Node::ImportMacro
        // Fail
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_errors_with_invalid_templates() {
        let res = crate::Template::init("{{ foo".to_string());
        assert!(res.is_err())
    }

    #[test]
    fn it_instantiates_with_valid_templates() {
        let res = crate::Template::init("{{ foo }}".to_string());
        assert!(res.is_ok())
    }

    #[test]
    fn it_returns_variables_with_basic_template() {
        let res = crate::Template::init("Hi {{ first_name }}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("first_name")]);
    }

    #[test]
    fn it_returns_variables_with_if() {
        let res = crate::Template::init("Hi {% if first_name %} hi {{ last_name }} {% else %} hi {{ email }} {% endif %}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("first_name"), String::from("last_name"), String::from("email")]);
    }

    #[test]
    fn it_returns_variables_with_fors_and_dot_prefix() {
        let res = crate::Template::init("{% for product in products %}{{loop.index}}. {{product.name}} {{ order_number }} {% endfor %}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("products"), String::from("order_number")]);
    }

    #[test]
    fn it_returns_variables_with_fors_and_bracket_prefix() {
        let res = crate::Template::init("{% for product in products %}{{loop.index}}. {{product['name']}} {{ order_number }} {% endfor %}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("products"), String::from("order_number")]);
    }

    #[test]
    fn it_returns_variables_with_fors_with_blacklist_test() {
        // add {{ product }} after blacklist loop - should be found.
        let res = crate::Template::init("{% for product in products %}{{loop.index}}. {{product.name}} {{ order_number }} {% endfor %} {{ product }}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("products"), String::from("order_number"), String::from("product")]);
    }

    #[test]
    fn it_ignores_set_vars() {
        let res = crate::Template::init("{{ name }} {% set uname = name %} {{ uname }}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("name")]);
    }

    #[test]
    fn it_captures_vars_with_no_blocks() {
        let res = crate::Template::init("{% set uname = name %}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("name")]);
    }

    #[test]
    fn it_captures_vars_with_filters() {
        let res = crate::Template::init("{% set uname = name | upper %}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("name")]);
    }

    #[test]
    fn it_captures_filters() {
        let res = crate::Template::init("{{  name | upper }}".to_string()).unwrap();
        let vars = res.get_variables();
        assert_eq!(vars, vec![String::from("name")]);
    }

    #[test]
    fn executing_templates() {
        let tpl = crate::Template::new("{{ name }}".to_string()).unwrap();
        let ctx = tpl.generate_context("{\"name\": \"mr bean\", \"products\": [{ \"sku\": 123}] }".into());
        assert!(ctx.is_ok(), "context generated");
        let res = tpl.execute_with_context(&ctx.unwrap());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "mr bean");
    }

    #[test]
    fn executing_templates_with_missing_vars() {
        let tpl = crate::Template::new("{{ name }}, {{ company }}{% for p in products %}{{ p.name }}{% endfor %}".to_string()).unwrap();
        let ctx = tpl.generate_context("{\"name\": \"mr bean\", \"products\": [{ \"sku\": 123 }] }".into());
        assert!(ctx.is_ok(), "context generated");
        let res = tpl.execute_with_context(&ctx.unwrap());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "mr bean, ");
    }

}
