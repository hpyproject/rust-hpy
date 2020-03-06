#!/usr/bin/env python3

import os
import sys
import hpy.universal

def immediate_out(s):
    sys.stdout.write(s)
    sys.stdout.flush()

project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
target = os.path.join(project_root, 'target', 'debug', 'examples')

def load(exmp_name):
    so_path = os.path.join(target, 'lib' + exmp_name + '.so')

    class Spec:
        name = exmp_name
        origin = so_path 

    return hpy.universal.load_from_spec(Spec)

mod = load('pof')
immediate_out("Testing a function call without arguments: ")
assert mod.do_nothing() is None
#print(mod.add_ints(5, 6))
#print(mod.double(21))

