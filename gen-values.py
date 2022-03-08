import math
import time

# Using an ellipse to emulate SS-RSRP values
#
#  x^2/a^2 + y^2/b^2 = 1
#
# a = 9
# b = 3
# Focal points
# F1 = 6√2
# F2 = -6√2
#

A=9
B=3
F1 = (-6*math.sqrt(2), 0)
F2 = (6*math.sqrt(2), 0)


def get_y(x):
    return B*math.sqrt((1-(pow(x,2)/pow(A,2))))

def distance(p1, p2):
    return math.sqrt( pow(p2[0] - p1[0],2) + pow(p2[1] - p1[1],2) )

def main():
    while True:
        for x in range(-90, 90, 1):
            x = x/10
            y = get_y(x)
            point=(x,y)
            dist_f1 = distance(F1, point)
            dist_f2 = distance(F2, point)

            rsrp_f1 = -10 * dist_f1
            rsrp_f2 = -10 * dist_f2

            print(f'########\n')
            print(f'Point ({x},{y})')
            print(f'Distance F1: {dist_f1}\n')
            print(f'Distance F2: {dist_f2}\n')
            print(f'RSRP F1: {rsrp_f1}\n')
            print(f'RSRP F2: {rsrp_f2}\n')
            print(f'########\n')
            time.sleep(0.5)


if __name__=='__main__':
    main()