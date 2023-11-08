import matplotlib.pyplot as plt
import numpy as np
from numpy.lib.stride_tricks import sliding_window_view

directory = "/linux/"  # os
elements_count = 15000  # elements


# SMA
def moving_average(data, window_size):
    # return np.convolve(data, np.ones(window_size) / window_size, mode="same")
    return np.average(sliding_window_view(data, window_shape=window_size), axis=1)


# EMA
def exponential_moving_average(data, alpha):
    ema = [data[0]]
    for i in range(1, len(data)):
        ema.append(alpha * data[i] + (1 - alpha) * ema[i - 1])
    return np.array(ema)


# WMA
def weighted_moving_average(data, weights):
    return np.convolve(data, weights, mode="valid")


def work(names):
    num_files = len(names)
    fig, axes = plt.subplots(1, num_files, figsize=(15, 5))
    for i, name in enumerate(names):
        with open("./input/" + name + ".txt", "r", encoding="utf-8") as file:
            data = list(map(int, file.read().split(" ")))
            data = np.array(data)
        print(name + " data len", len(data))
        lower = np.percentile(data, 2)
        upper = np.percentile(data, 95)
        data = data[(data < upper) * (data > lower)]
        print(name + " data len after cleaning", len(data))
        print(name + " mean: ", np.mean(data))
        data = moving_average(data, 100)
        print()

        axes[i].plot(data)
        axes[i].set_title(name)

    plt.tight_layout()
    plt.savefig(
        "./results"
        + directory
        + names[0][0:-1]
        + "combined"
        + "_"
        + str(elements_count)
        + ".png"
    )
    plt.clf()


def work_combined(names):
    fig, ax = plt.subplots(figsize=(15, 5))

    for name in names:
        with open("./input/" + name + ".txt", "r", encoding="utf-8") as file:
            data = list(map(int, file.read().split(" ")))

        data = np.array(data)
        print(name + " data len", len(data))
        lower = np.percentile(data, 2)
        upper = np.percentile(data, 95)
        data = data[(data < upper) * (data > lower)]
        print(name + " data len after cleaning", len(data))
        print(name + " mean: ", np.mean(data))
        data = moving_average(data, 30)

        # Plot data on the same subplot
        ax.plot(data, label=name)

    ax.set_title("Combined Plot")
    ax.legend(loc="upper right")

    # Save the combined plot
    plt.tight_layout()
    plt.savefig(
        "./results"
        + directory
        + names[0][0:-1]
        + "merged"
        + "_"
        + str(elements_count)
        + ".png"
    )
    plt.clf()


# List of file names to process
writes = sorted(
    [
        "write_times_write_1",
        "write_times_write_2",
        "write_times_write_3",
        "write_times_write_0",
    ]
)
opens = sorted(
    [
        "open_times_write_1",
        "open_times_write_2",
        "open_times_write_3",
        "open_times_write_0",
    ]
)
finishes = sorted(
    [
        "finish_times_write_1",
        "finish_times_write_2",
        "finish_times_write_3",
        "finish_times_write_0",
    ]
)

work(writes)
work(opens)
work(finishes)
work_combined(writes)
work_combined(opens)
work_combined(finishes)
