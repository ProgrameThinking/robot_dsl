use crate::ast::{self, Expr, Statement};
use crate::tokens::Token;
use std::collections::HashMap;
use std::io;

pub struct Interpreter {
    /* 解释器运行的环境，实现较为简单。
     * 本解释器唯一需要注意的是variable和value之间的一一对应
     * 因此可以环境内部便是一个map，key=variable，value=value
     * 环境之间管理采用stack结构，
     * 当进入一个block时可以将父环境拷贝，然后处理新增变量
     * 执行完该block后可以直接pop掉
     */
    pub env: Vec<HashMap<String, String>>,
    pub ast: Vec<Box<ast::Statement>>,
}

impl Interpreter {
    /* 创建解释器对象 */
    pub fn new(ast: Vec<Box<ast::Statement>>) -> Self {
        let env = vec![HashMap::new()];
        Self { env, ast }
    }
    /* 添加新环境 */
    fn add_new_env(&mut self) {
        let new_env = self.env.last().cloned().unwrap(); // 克隆上一个环境
        self.env.push(new_env);
    }
    /* 从栈顶弹出当前环境 */
    fn rm_now_env(&mut self) {
        self.update_env();
        self.env.pop();
    }
    /* 添加变量到当前环境中 */
    fn add_new_var(&mut self, name: String, init: Box<ast::Expr>) {
        if let Some(cur_env) = self.env.last_mut() {
            cur_env.insert(name, init.trans(cur_env.clone()));
        }
    }

    pub fn update_env(&mut self) {
        if self.env.len() < 2 {
            return; // 如果环境中的 HashMap 小于 2 个，无法进行比较
        }

        let last_idx = self.env.len() - 1;
        let second_last_idx = last_idx - 1;

        let last_env = self.env.get(last_idx).cloned();
        let second_last_env = self.env.get_mut(second_last_idx);

        if let (Some(last_env), Some(second_last_env)) = (last_env, second_last_env) {
            // 创建一个克隆的 HashMap，用于存储需要更新的键值对
            let updates: HashMap<String, String> = last_env
                .iter()
                .filter(|(key, value)| {
                    // 仅保留与倒数第二个 HashMap 不同的项
                    second_last_env.get(*key) != Some(value)
                })
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect();

            // 更新倒数第二个 HashMap
            second_last_env.extend(updates);
        }
    }

    // /* 查找并返回变量的值 */
    // fn get_variable(&self, name: &str) -> Option<String> {
    //     if let Some(cur_env) = self.env.last() {
    //         if let Some(value) = cur_env.get(name) {
    //             return Some(value.clone());
    //         }
    //     }
    //     None // 未找到变量
    // }

    pub fn interpret(&mut self) {
        let mut new_ast = Vec::new();
        std::mem::swap(&mut self.ast, &mut new_ast);
        for statement in new_ast {
            self.execute(statement);
        }
    }

    fn execute(&mut self, statement: Box<Statement>) {
        match *statement {
            ast::Statement::Speak { expression } => {
                println!("{}", expression.trans(self.env.last().unwrap().clone()));
            }
            ast::Statement::Var { name, init } => {
                self.add_new_var(name, init);
            }
            ast::Statement::Loop { body } => loop {
                self.execute(body.clone());
            },
            ast::Statement::Input { input } => {
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("无法读取输入");
                // 去除回车符
                value = value.trim().to_string();
                self.add_new_var(
                    input,
                    Box::new(Expr::Literal {
                        value: ast::LiteralValue::String(value),
                    }),
                );
            }
            ast::Statement::Expression { expression } => {
                expression.exec(self.env.last_mut().unwrap());
            }
            ast::Statement::Exit => {
                std::process::exit(0);
            }
            ast::Statement::Branch { condition, then } => {
                if let Expr::Literal { value: res } = *condition.exec(self.env.last_mut().unwrap())
                {
                    match res {
                        ast::LiteralValue::String(value) => {
                            if value == "True" {
                                self.execute(then);
                            } else if value == "False" {
                            } else {
                                panic!("结果非布尔值！")
                            }
                        }
                        _ => {
                            panic!("结果非布尔值！")
                        }
                    }
                } else {
                    panic!("结果非布尔值！")
                }
            }
            ast::Statement::Block { statements } => {
                self.add_new_env();
                for stmt in statements {
                    self.execute(stmt);
                }
                self.rm_now_env();
            }
        }
    }
}

pub trait Utils {
    /*
     * 用于将Expr类型转换为可以输出的字符串
     * 为interpreter.rs服务，因此只需要建立接口
     * 具体实现依靠interpreter.rs中的环境实现
     */
    fn trans(&self, env: HashMap<String, String>) -> String;
    /*
     * 用于对表达式语句的执行
     */
    fn exec(&self, env: &mut HashMap<String, String>) -> Box<Expr>;
}

impl Utils for Box<Expr> {
    fn trans(&self, env: HashMap<String, String>) -> String {
        match &**self {
            Expr::Assign { .. } => {
                panic!("本程序不允许采用连等式!");
            }
            Expr::Binary { .. } => {
                let mut cur_env = env.clone();
                if let Expr::Literal { value: res } = *self.exec(&mut cur_env) {
                    match res {
                        ast::LiteralValue::String(value) => {
                            format!("{}", value)
                        }
                        ast::LiteralValue::Number(value) => {
                            format!("{}", value.to_string())
                        }
                    }
                } else {
                    panic!("结果应为字符串!")
                }
            }
            Expr::Literal { value } => value.trans(),
            Expr::Variable { name } => {
                if let Some(value) = env.get(name) {
                    value.clone()
                } else {
                    panic!("Variable '{}' cannot be found", name);
                }
            }
        }
    }
    fn exec(&self, env: &mut HashMap<String, String>) -> Box<Expr> {
        match &**self {
            Expr::Assign { name, value } => {
                env.insert(name.clone(), value.trans(env.clone()));
                Box::new(Expr::Literal {
                    value: ast::LiteralValue::String("Successd".to_string()),
                })
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // 首先计算左表达式和右表达式的值
                let left_value = left.exec(env);
                let right_value = right.exec(env);
                match operator {
                    Token::OperatorAdd => {
                        // 先解包 left_value 和 right_value
                        if let Expr::Literal {
                            value: left_literal,
                        } = *left_value
                        {
                            if let Expr::Literal {
                                value: right_literal,
                            } = *right_value
                            {
                                match (left_literal, right_literal) {
                                    (
                                        ast::LiteralValue::Number(left_num),
                                        ast::LiteralValue::Number(right_num),
                                    ) => {
                                        //处理两个数字的情况
                                        let result = left_num + right_num;
                                        Box::new(Expr::Literal {
                                            value: (ast::LiteralValue::Number(result)),
                                        })
                                    }
                                    (
                                        ast::LiteralValue::String(left_str),
                                        ast::LiteralValue::String(right_str),
                                    ) => {
                                        match (left_str.parse::<f64>(), right_str.parse::<f64>()) {
                                            (Ok(left_num), Ok(right_num)) => {
                                                // 处理两个数字的情况
                                                let result = left_num + right_num;
                                                Box::new(Expr::Literal {
                                                    value: ast::LiteralValue::Number(result),
                                                })
                                            }
                                            _ => {
                                                // 处理字符串拼接
                                                let result = format!("{}{}", left_str, right_str);
                                                Box::new(Expr::Literal {
                                                    value: ast::LiteralValue::String(result),
                                                })
                                            }
                                        }
                                    }
                                    (
                                        ast::LiteralValue::String(left_str),
                                        ast::LiteralValue::Number(right_num),
                                    ) => {
                                        // 为字符串和数字
                                        let result =
                                            format!("{}{}", left_str, right_num.to_string());
                                        Box::new(Expr::Literal {
                                            value: (ast::LiteralValue::String(result)),
                                        })
                                    }
                                    (
                                        ast::LiteralValue::Number(left_num),
                                        ast::LiteralValue::String(right_str),
                                    ) => {
                                        // 为字符串和数字
                                        let result =
                                            format!("{}{}", left_num.to_string(), right_str);
                                        Box::new(Expr::Literal {
                                            value: (ast::LiteralValue::String(result)),
                                        })
                                    }
                                }
                            } else {
                                panic!("Right value is not a Literal ");
                            }
                        } else {
                            panic!("Left value is not a Literal");
                        }
                    }
                    Token::OperatorSub => {
                        if let Expr::Literal {
                            value: left_literal,
                        } = *left_value
                        {
                            if let Expr::Literal {
                                value: right_literal,
                            } = *right_value
                            {
                                match (left_literal, right_literal) {
                                    (
                                        ast::LiteralValue::Number(left_num),
                                        ast::LiteralValue::Number(right_num),
                                    ) => {
                                        //处理两个数字的情况
                                        let result = left_num - right_num;
                                        Box::new(Expr::Literal {
                                            value: ast::LiteralValue::Number(result),
                                        })
                                    }
                                    (
                                        ast::LiteralValue::String(left_str),
                                        ast::LiteralValue::String(right_str),
                                    ) => {
                                        // 尝试将字符串解析为数字，如果成功，则执行数字减法
                                        if let Ok(left_num) = left_str.parse::<f64>() {
                                            if let Ok(right_num) = right_str.parse::<f64>() {
                                                let result = left_num - right_num;
                                                Box::new(Expr::Literal {
                                                    value: ast::LiteralValue::Number(result),
                                                })
                                            } else {
                                                panic!(
                                                    "Right value and left value must be a number!"
                                                );
                                            }
                                        } else {
                                            panic!("Right value and left value must be a number!");
                                        }
                                    }
                                    _ => {
                                        panic!("Right value and left value must be a number!");
                                    }
                                }
                            } else {
                                panic!("Right value is not a Literal ");
                            }
                        } else {
                            panic!("Left value is not a Literal");
                        }
                    }
                    Token::OperatorMul => {
                        if let Expr::Literal {
                            value: left_literal,
                        } = *left_value
                        {
                            if let Expr::Literal {
                                value: right_literal,
                            } = *right_value
                            {
                                match (left_literal, right_literal) {
                                    (
                                        ast::LiteralValue::Number(left_num),
                                        ast::LiteralValue::Number(right_num),
                                    ) => {
                                        //处理两个数字的情况
                                        let result = left_num * right_num;
                                        Box::new(Expr::Literal {
                                            value: ast::LiteralValue::Number(result),
                                        })
                                    }
                                    (
                                        ast::LiteralValue::String(left_str),
                                        ast::LiteralValue::String(right_str),
                                    ) => {
                                        // 尝试将字符串解析为数字，如果成功，则执行数字乘法
                                        if let Ok(left_num) = left_str.parse::<f64>() {
                                            if let Ok(right_num) = right_str.parse::<f64>() {
                                                let result = left_num * right_num;
                                                Box::new(Expr::Literal {
                                                    value: ast::LiteralValue::Number(result),
                                                })
                                            } else {
                                                panic!(
                                                    "Right value and left value must be a number!"
                                                );
                                            }
                                        } else {
                                            panic!("Right value and left value must be a number!");
                                        }
                                    }
                                    _ => {
                                        panic!("Right value and left value must be a number!");
                                    }
                                }
                            } else {
                                panic!("Right value is not a Literal ");
                            }
                        } else {
                            panic!("Left value is not a Literal");
                        }
                    }
                    Token::OperatorDiv => {
                        if let Expr::Literal {
                            value: left_literal,
                        } = *left_value
                        {
                            if let Expr::Literal {
                                value: right_literal,
                            } = *right_value
                            {
                                match (left_literal, right_literal) {
                                    (
                                        ast::LiteralValue::Number(left_num),
                                        ast::LiteralValue::Number(right_num),
                                    ) => {
                                        //处理两个数字的情况
                                        if right_num != 0.0 {
                                            let result = left_num / right_num;
                                            Box::new(Expr::Literal {
                                                value: ast::LiteralValue::Number(result),
                                            })
                                        } else {
                                            panic!("Division by zero is not allowed!");
                                        }
                                    }
                                    (
                                        ast::LiteralValue::String(left_str),
                                        ast::LiteralValue::String(right_str),
                                    ) => {
                                        // 尝试将字符串解析为数字，如果成功，则执行数字除法
                                        if let Ok(left_num) = left_str.parse::<f64>() {
                                            if let Ok(right_num) = right_str.parse::<f64>() {
                                                if right_num != 0.0 {
                                                    let result = left_num / right_num;
                                                    Box::new(Expr::Literal {
                                                        value: ast::LiteralValue::Number(result),
                                                    })
                                                } else {
                                                    panic!("Division by zero is not allowed!");
                                                }
                                            } else {
                                                panic!(
                                                    "Right value and left value must be a number!"
                                                );
                                            }
                                        } else {
                                            panic!("Right value and left value must be a number!");
                                        }
                                    }
                                    _ => {
                                        panic!("Right value and left value must be a number!");
                                    }
                                }
                            } else {
                                panic!("Right value is not a Literal ");
                            }
                        } else {
                            panic!("Left value is not a Literal");
                        }
                    }
                    Token::WEqual => {
                        if let Expr::Literal {
                            value: left_literal,
                        } = *left_value
                        {
                            if let Expr::Literal {
                                value: right_literal,
                            } = *right_value
                            {
                                match (left_literal, right_literal) {
                                    (
                                        ast::LiteralValue::String(left_str),
                                        ast::LiteralValue::String(right_str),
                                    ) => {
                                        // 处理两个字符串的情况
                                        if left_str == right_str {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "True".to_string(),
                                                )),
                                            })
                                        } else {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "False".to_string(),
                                                )),
                                            })
                                        }
                                    }
                                    (
                                        ast::LiteralValue::String(left_str),
                                        ast::LiteralValue::Number(right_num),
                                    ) => {
                                        // 为字符串和数字
                                        if left_str == right_num.to_string() {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "True".to_string(),
                                                )),
                                            })
                                        } else {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "False".to_string(),
                                                )),
                                            })
                                        }
                                    }
                                    (
                                        ast::LiteralValue::Number(left_num),
                                        ast::LiteralValue::String(right_str),
                                    ) => {
                                        // 为字符串和数字
                                        if left_num.to_string() == right_str {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "True".to_string(),
                                                )),
                                            })
                                        } else {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "False".to_string(),
                                                )),
                                            })
                                        }
                                    }
                                    (
                                        ast::LiteralValue::Number(left_num),
                                        ast::LiteralValue::Number(right_num),
                                    ) => {
                                        //处理两个数字的情况
                                        if left_num == right_num {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "True".to_string(),
                                                )),
                                            })
                                        } else {
                                            Box::new(Expr::Literal {
                                                value: (ast::LiteralValue::String(
                                                    "False".to_string(),
                                                )),
                                            })
                                        }
                                    }
                                }
                            } else {
                                panic!("Right value is not a Literal ");
                            }
                        } else {
                            panic!("Left value is not a Literal");
                        }
                    }
                    _ => {
                        panic!("Need a operator!");
                    }
                }
            }
            Expr::Literal { value } => match value {
                ast::LiteralValue::Number(num) => Box::new(Expr::Literal {
                    value: ast::LiteralValue::Number(*num),
                }),
                ast::LiteralValue::String(str) => Box::new(Expr::Literal {
                    value: ast::LiteralValue::String(str.to_string()),
                }),
            },
            Expr::Variable { name } => {
                let res = env.get(name).unwrap();
                Box::new(Expr::Literal {
                    value: ast::LiteralValue::String(res.to_string()),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /*
     * 对声明变量方法进行测试
     * 对变量连加进行测试
     */
    fn test_addition() {
        // 创建一个Interpreter实例
        let mut interpreter = Interpreter::new(vec![]);
        let mut env = interpreter.env.last_mut().unwrap();

        // 添加一个变量 name 到环境中
        let name_variable_name = "name".to_string();
        let name_variable_value = "Tom".to_string();
        let name_var_name = "name".to_string();
        let name_var_value = Box::new(Expr::Literal {
            value: ast::LiteralValue::String(name_variable_value.clone()),
        });
        let name_var_assign_expr = Box::new(Expr::Assign {
            name: name_var_name,
            value: name_var_value,
        });

        // 调用 exec 方法来声明变量
        name_var_assign_expr.exec(&mut env);

        // 创建一个带变量的字符串连接表达式
        let left_value = Box::new(Expr::Literal {
            value: ast::LiteralValue::String("你好".to_string()),
        });
        let middle_value = Box::new(Expr::Variable {
            name: name_variable_name.clone(),
        });
        let right_value = Box::new(Expr::Literal {
            value: ast::LiteralValue::String("，请问有什么需要帮助的？".to_string()),
        });
        let concat_expr = Box::new(Expr::Binary {
            left: left_value,
            operator: Token::OperatorAdd,
            right: Box::new(Expr::Binary {
                left: middle_value,
                operator: Token::OperatorAdd,
                right: right_value,
            }),
        });
        // 调用 exec 方法并检查返回值
        let result = concat_expr.exec(&mut env);
        // 检查结果是否是预期的 LiteralValue::String("你好，Tom请问有什么需要帮助的？")
        assert_eq!(
            result,
            Box::new(Expr::Literal {
                value: (ast::LiteralValue::String("你好Tom，请问有什么需要帮助的？".to_string())),
            })
        );
    }

    #[test]
    fn test_complex_equality() {
        // 创建一个Interpreter实例
        let mut interpreter = Interpreter::new(vec![]);
        let mut env = interpreter.env.last_mut().unwrap();

        // 创建一个变量并赋值
        let variable_name = "x".to_string();
        let variable_value = "Hello".to_string();
        env.insert(variable_name.clone(), variable_value.clone());

        // 创建左操作数的表达式
        let left_value = Box::new(Expr::Variable {
            name: variable_name.clone(),
        });

        // 创建右操作数的表达式
        let right_value = Box::new(Expr::Literal {
            value: ast::LiteralValue::String("Hello".to_string()),
        });

        // 创建一个相等性比较表达式
        let eq_expr = Box::new(Expr::Binary {
            left: left_value,
            operator: Token::WEqual,
            right: right_value,
        });

        // 调用 exec 方法并检查返回值
        let result = eq_expr.exec(&mut env);

        // 检查结果是否是预期的 LiteralValue::String("True")
        assert_eq!(
            result,
            Box::new(Expr::Literal {
                value: (ast::LiteralValue::String("True".to_string())),
            })
        );
    }
}
