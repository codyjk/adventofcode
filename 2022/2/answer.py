#!/usr/bin/env python3
ROCK = 1
PAPER = 2
SCISSORS = 3

WIN = 6
LOSE = 0
DRAW = 3

OPPONENT_CYPHER = {
    'A': ROCK,
    'B': PAPER,
    'C': SCISSORS,
}

SELF_CYPHER = {
    'X': ROCK,
    'Y': PAPER,
    'Z': SCISSORS,
}

SELF_CYPHER_2 = {
    'X': LOSE,
    'Y': DRAW,
    'Z': WIN,
}

INFERRED_MOVE_MAP = {
    (ROCK, WIN): PAPER,
    (ROCK, DRAW): ROCK,
    (ROCK, LOSE): SCISSORS,
    (PAPER, WIN): SCISSORS,
    (PAPER, DRAW): PAPER,
    (PAPER, LOSE): ROCK,
    (SCISSORS, WIN): ROCK,
    (SCISSORS, DRAW): SCISSORS,
    (SCISSORS, LOSE): PAPER,
}


def defeats(one, two):
    if one == ROCK and two == SCISSORS:
        return True
    elif one == PAPER and two == ROCK:
        return True
    elif one == SCISSORS and two == PAPER:
        return True
    else:
        return False

score = 0

with open('input.txt') as file:
    for line in file:
        if not line.strip():
            continue
        opponent_enc, me_enc = line.split(' ')
        opponent = OPPONENT_CYPHER[opponent_enc.strip()]
        me = SELF_CYPHER[me_enc.strip()]
        if me == opponent:
            score += me + DRAW
        elif defeats(me, opponent):
            score += me + WIN
        else:
            score += me + LOSE

print('final score: {}'.format(score))

score2 = 0
with open('input.txt') as file:
    for line in file:
        if not line.strip():
            continue
        opponent_enc, desired_outcome_enc = line.split(' ')
        opponent = OPPONENT_CYPHER[opponent_enc.strip()]
        desired_outcome = SELF_CYPHER_2[desired_outcome_enc.strip()]
        inferred_move = INFERRED_MOVE_MAP[(opponent, desired_outcome)]
        score2 += inferred_move + desired_outcome

print('final score2: {}'.format(score2))
