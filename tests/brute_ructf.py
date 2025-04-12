#!/usr/bin/env python3

import random
import string
import sys

print("Hello! I am a little sploit. I could be written on any language, but "
      "my author loves Python. Look at my source - it is really simple. "
      "I should steal flags and print them on stdout or stderr. ")

host = sys.argv[1]
print("I need to attack a team with host: {}".format(host))

print("Here are the flags in the required format:")

team_ids = ["TEAM003", "TEAM002"]

for team_id in team_ids:
    random_part = ''.join(random.choice(string.ascii_uppercase + string.digits) for _ in range(32))
    
    flag = f"{team_id}_{random_part}"
    print(flag, flush=True)