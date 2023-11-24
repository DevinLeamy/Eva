import math


def vec3_add(a: [float], b: [float]) -> [float]:
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def vec3_sub(a: [float], b: [float]) -> [float]:
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def vec3_scalar_mult(a: [float], s: float) -> [float]:
    return [a[0] * s, a[1] * s, a[2] * s]


def vec3_mult(a: [float], b: [float]) -> [float]:
    return [a[0] * b[0], a[1] * b[1], a[2] * b[2]]


def vec3_normalize(v: [float]) -> [float]:
    vv = vec3_mult(v, v)
    length = math.sqrt(vv[0] + vv[1] + vv[2])
    return [v[0] / length, v[1] / length, v[2] / length]
