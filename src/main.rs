//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Lesser General Public License as published by
//     the Free Software Foundation, version 3 of the License.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Lesser General Public License for more details.

//     You should have received a copy of the GNU Lesser General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::VecDeque;

mod args_parser;
mod top_pss;

const VERSION: &str = "0.4";

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();

    // First argument is a program name. We do not need it
    let _self = args.pop_front();

    let program_args = args_parser::ProgramArgs::parse_args(args);

    if program_args.first_n == 0 {
        return;
    }

    top_pss::toprss(program_args);
}

fn help() {
    version();
    println!(
        r#"
        usage:
            toppss
            toppss [options]

Command line utility for printing top ram processes

options:
  -h, --help, -H, -?                 display this help message and exit
  -v, --version                      display program's version and exit
      --collapse-count                  display count of collapsed processes
  -c, --uncollapse                   uncollapse processes with the same name
  -n,                DEFAULT n = 3   display first 'n' processes
  -a, --all                          display all processes 
  -l, --lines                        display each process on separate line
      --kb                           display ram usage in kB
      --mb                           display ram usage in MB
      --gb                           display ram usage in GB
    "#
    )
}

fn version() {
    println!("Toppss version: {VERSION}");
}
