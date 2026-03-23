//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Lesser General Public License as published by
//     the Free Software Foundation, version 3 of the License.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Lesser General Public License for more details.

//     You should have received a copy of the GNU Lesser General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    collections::{VecDeque, vec_deque::Iter},
    io::Error,
    path::PathBuf,
    process::exit,
    usize,
};

use crate::{
    help,
    top_pss::{Separator, Unit},
    version,
};
pub struct ProgramArgs {
    pub collapse: bool,
    pub first_n: usize,
    pub separator: Separator,
    pub unit: Option<Unit>,
    pub show_group_count: bool,
    pub path: PathBuf,
}

impl ProgramArgs {
    pub fn parse_args(args: VecDeque<String>) -> ProgramArgs {
        Self::match_args(args)
    }

    fn match_args(args: VecDeque<String>) -> ProgramArgs {
        let mut args_iter = args.iter();
        let mut program_args_builder = ProgramArgsBuilder::new();

        while let Some(arg) = args_iter.next() {
            if arg.starts_with("--") {
                if let Some(arg) = arg.get(2..) {
                    Self::match_long_arg(arg, &mut args_iter, &mut program_args_builder);
                }
            } else if arg.starts_with("-") {
                let mut arg_cp = arg.chars();
                let _ = arg_cp.next();
                for c in arg_cp {
                    Self::match_short_arg(&c, &mut args_iter, &mut program_args_builder);
                }
            } else {
                eprintln!("Error: Invalid argument: {}", arg);
                exit(0);
            }
        }

        program_args_builder.build()
    }

    fn match_long_arg(
        arg: &str,
        args_iter: &mut Iter<'_, String>,
        args_builder: &mut ProgramArgsBuilder,
    ) {
        match arg {
            "help" => {
                help();
                exit(1);
            }

            "ungroup" => {
                args_builder.set_collapse(false);
            }

            "version" => {
                version();
                exit(1);
            }

            "kb" => {
                args_builder.set_unit(Some(Unit::kB));
            }

            "mb" => {
                args_builder.set_unit(Some(Unit::MB));
            }

            "gb" => {
                args_builder.set_unit(Some(Unit::GB));
            }

            "lines" => {
                args_builder.set_separator(Separator::Lines);
            }

            "all" => {
                args_builder.set_n(usize::MAX);
            }

            "group-count" => {
                args_builder.set_show_group_count(true);
            }

            "custom-separator" => {
                let expected_custom_separator = args_iter.next();
                if let Some(separator) = expected_custom_separator {
                    args_builder.set_separator(Separator::Other(separator.clone()));
                } else {
                    eprintln!("Error: found option '-n', but no number was provided");
                    exit(0);
                }
            }

            _ => {
                eprintln!("Error: Unknown argument '{arg}'");
                exit(0);
            }
        }
    }

    fn match_short_arg(
        arg: &char,
        args_iter: &mut Iter<'_, String>,
        args_builder: &mut ProgramArgsBuilder,
    ) {
        match arg {
            'h' | 'H' | '?' => {
                help();
                exit(1);
            }

            'v' => {
                version();
                exit(1);
            }

            'u' => {
                args_builder.set_collapse(false);
            }

            'n' => {
                let expected_number = args_iter.next();
                if let Some(number) = expected_number {
                    match number.parse::<usize>() {
                        Ok(n) => {
                            args_builder.set_n(n);
                        }
                        Err(_) => {
                            eprintln!("Error: Could not parse '{number}' into unsigned integer");
                            exit(0);
                        }
                    }
                } else {
                    eprintln!("Error: found option '-n', but no number was provided");
                    exit(0);
                }
            }

            'a' => {
                args_builder.set_n(usize::MAX);
            }
            _ => {
                eprintln!("Error: Unknown argument '{arg}'");
                exit(0);
            }
        }
    }

    // fn match_long_arg(&mut self, arg: &str) {
    //     match arg {

    //         "--run-tests-this-option-is-hidden-and-intended-to-be-used-to-perform-tests-by-developer-this-option-name-is-annoingly-long-for-a-reason" =>
    //         {
    //             let expected_new_proc_path = args_iter.next();
    //             if let Some(p) = expected_new_proc_path {
    //                 let new_path = PathBuf::from(p);
    //                 if new_path.exists() {
    //                     path = new_path;
    //                 } else {
    //                     eprintln!("Error: Path '{}' does not exists", new_path.display());
    //                     return;
    //                 }
    //             } else {
    //                 eprintln!(
    //                     "Error: found option '--run-tests-this-option-is-hidden-and-intended-to-be-used-to-perform-tests-by-developer-this-option-name-is-annoingly-long-for-a-reason', but no path was provided"
    //                 );
    //                 return;
    //             }
    //         }
    //         _ => {
    //             eprintln!("Error: Unknown argument '{arg}'");
    //             return;
    //         }
    //     }
    // }
}

struct ProgramArgsBuilder {
    collapse: bool,
    first_n: usize,
    separator: Separator,
    unit: Option<Unit>,
    show_group_count: bool,
    path: PathBuf,
}

impl ProgramArgsBuilder {
    pub fn new() -> Self {
        ProgramArgsBuilder {
            collapse: true,
            first_n: 3,
            separator: Separator::Line,
            unit: None,
            show_group_count: false,
            path: PathBuf::from("/proc"),
        }
    }

    pub fn set_collapse(&mut self, collapse: bool) {
        self.collapse = collapse;
    }

    pub fn set_n(&mut self, first_n: usize) {
        self.first_n = first_n;
    }

    pub fn set_separator(&mut self, separator: Separator) {
        self.separator = separator;
    }

    pub fn set_unit(&mut self, unit: Option<Unit>) {
        self.unit = unit;
    }

    pub fn set_show_group_count(&mut self, show_group_count: bool) {
        self.show_group_count = show_group_count;
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    pub fn build(self) -> ProgramArgs {
        ProgramArgs {
            collapse: self.collapse,
            first_n: self.first_n,
            separator: self.separator,
            unit: self.unit,
            show_group_count: self.show_group_count,
            path: self.path,
        }
    }
}
