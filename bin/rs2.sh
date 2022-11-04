#!/bin/sh
#####################################################

script_abs=$(readlink -f "$0")
script_dir=$(dirname $script_abs)
base_dir=$(igo rs)

java -Djava.awt.headless=true -classpath $base_dir/bin/rulp_shell_bin_0.0.1.jar main.DoMain $*

