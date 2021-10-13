#!/usr/bin/env python3
import matplotlib.pyplot as plt

dt = 10
G = 9.81
sample_rate = 1
n_samples = 100

def integration(t):
    return (G * t**2)/2

def euler_integration():
    p = 0.0
    v = 0.0
    while True:
        v += G * dt
        p += v * dt
        yield p

def main():
    x_axis = [0.0]
    normal_y = [0.0]
    euler_y = [0.0]

    i = 1
    sample_count = 0
    int_gen = euler_integration()
    while True:
        r1 = integration(i * dt)
        r2 = next(int_gen)
        if i % sample_rate == 0:
            x_axis.append(i * dt)
            normal_y.append(r1)
            euler_y.append(r2)
            sample_count += 1
        if sample_count == n_samples:
            break
        i += 1
    delta = [euler_y[i] - normal_y[i] for i in range(len(x_axis))]

    plt.xlabel(u"t in s (∆t = {}s)".format(dt))
    plt.ylabel("x in m")
    plt.ylim(0.0, 50000)
    plt.xlim(0.0, 100)
    plt.plot(x_axis, normal_y, 'b', label="Normale Integration")
    plt.plot(x_axis, euler_y, 'r', label="Euler Integration")
    plt.plot(x_axis, delta, 'black', label="∆x")
    plt.legend()
    plt.show()

if __name__ == '__main__':
    main()

# a
# a * t + v0
# a/2 * t^2 + v0 * t + p0
