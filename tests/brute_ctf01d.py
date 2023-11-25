#!/usr/bin/env python3

import random
import string
import sys

print("Hello! I am a little sploit. I could be written on any language, but "
      "my author loves Python. Look at my source - it is really simple. "
      "I should steal flags and print them on stdout or stderr. ")

host = sys.argv[1]
print("I need to attack a team with host: {}".format(host))

print("Here are some random flags for you:")

for _ in range(3):
    flag = 'c01d'+''.join(random.choice(string.ascii_lowercase + string.digits) for _ in range(4)) + '-' + ''.join(random.choice(string.ascii_lowercase + string.digits) for _ in range(4)) + '-' + ''.join(random.choice(string.ascii_lowercase + string.digits) for _ in range(4))  + '-' + ''.join(random.choice(string.ascii_lowercase + string.digits) for _ in range(4)) + '-' + ''.join(random.choice(string.ascii_lowercase + string.digits) for _ in range(4)) + ''.join(random.choice(string.digits) for _ in range(8))
    print(flag, flush=True)
