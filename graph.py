import matplotlib.pyplot as plt
import numpy as np

def moving_average(x, w):
    return np.convolve(x, np.ones(w), 'valid') / w

def work(name: str):
    with open(name+'.txt', "r", encoding="utf-8") as file:
        data = list(map(lambda x: int(x), file.read().split(" ")))

    data = np.array(data)
    print("data len", len(data))
    lower = np.percentile(data, 2)
    upper = np.percentile(data, 95)
    data = data[(data < upper) * (data > lower)]
    print("data len after cleaning", len(data))
    print(name+" mean: ", np.mean(data))
    data = moving_average(data, 30)
    plt.plot(data)
    plt.savefig(name+'.png')
    plt.clf()
tail = ""
work("open_times"+tail)
work("write_times"+tail)
work("finish_times"+tail)

