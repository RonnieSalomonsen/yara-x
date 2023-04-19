use crate::modules::prelude::*;
use crate::modules::protos::string::*;

#[module_main]
fn main(_ctx: &ScanContext) -> String {
    // Nothing to do, but we have to return our protobuf
    let string_proto = String::new();
    string_proto
}

#[module_export(name = "to_int")]
fn to_int(ctx: &ScanContext, string: RuntimeString) -> Option<i64> {
    let string = string.as_bstr(ctx);

    match string.to_str() {
        Ok(n) => return Some(n.parse::<i64>().unwrap()),
        Err(_) => return None,
    }
}

#[module_export(name = "to_int")]
fn to_int_base(
    ctx: &ScanContext,
    string: RuntimeString,
    base: i64,
) -> Option<i64> {
    let string = string.as_bstr(ctx);

    if base < 2 || base > 36 {
        return None;
    }
    let strobj = match string.to_str() {
        Ok(n) => n,
        Err(_) => return None,
    };
    match i64::from_str_radix(strobj, base.try_into().unwrap()) {
        Ok(n) => {
            return Some(n);
        }
        Err(_) => return None,
    }
}

#[module_export]
fn length(ctx: &ScanContext, string: RuntimeString) -> Option<i64> {
    return Some(string.as_bstr(ctx).len().try_into().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn end2end() {
        let rules = crate::compiler::Compiler::new()
            .add_source(
                r#"import "string"
                // True
                rule rule_1 { condition: string.length("AXsx00ERS") == 9 }
                rule rule_2 { condition: string.length("AXsx00ERS") == 9 }
                // False
                rule rule_3 { condition: string.length("AXsx00ERS") > 9 }
                rule rule_4 { condition: string.length("AXsx00ERS") < 9 }


                // True
                rule rule_5 { condition: string.to_int("1234") == 1234 }
                rule rule_6 { condition: string.to_int("-10") == -10 }
                // False
                rule rule_7 { condition: string.to_int("-10") == -8 }
                

                // True
                rule rule_8 { condition: string.to_int("A", 16) == 10 }
                rule rule_9 { condition: string.to_int("011", 8) == 9 }
                // False
                rule rule_10 { condition: string.to_int("-011", 0) == -9 }
                "#,
            )
            .unwrap()
            .build()
            .unwrap();

        let mut scanner = crate::scanner::Scanner::new(&rules);

        assert_eq!(scanner.scan(&[]).num_matching_rules(), 6);
    }
}
