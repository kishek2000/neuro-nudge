import json
import matplotlib.pyplot as plt
import numpy as np

while True:
    # Ask for which strategy to plot
    strategy = input(">> NeuroNudge Plots: Which strategy (1,2,3,4) would you like to plot?: ")
    while strategy not in ["1", "2", "3", "4", "q"]:
        strategy = input(">> NeuroNudge Plots: Which strategy (1,2,3,4) would you like to plot?: ")
        
    if strategy == "q" or strategy == "Q":
        print(">> NeuroNudge Plots: Exiting...")
        break

    # Load JSON data
    with open(f'./engine/strategy_{strategy}_simulation_results.json', 'r') as file:
        data = json.load(file)

    learner_ids = ["Learner 1", "Learner 2", "Learner 3", "Learner 4", "Learner 5", "Learner 6"]
    difficulty_order = ["VeryEasy", "Easy", "Medium", "Hard", "VeryHard", "Expert", "Master", "Grandmaster"]

    # Organize the data
    learner_progress = {learner: {difficulty: [] for difficulty in difficulty_order} for learner in learner_ids}
    difficulty_levels_attempted = {learner: [] for learner in learner_ids}  # Store attempted difficulty levels

    for iteration in data['iterations']:
        for learner in iteration['values']:
            learner_id = learner['learner_id']
            difficulty_levels_attempted[learner_id].append(learner['difficulty_level'])  # Get the attempted difficulty level
            for difficulty, q_value in learner['values'].items():
                learner_progress[learner_id][difficulty].append(q_value)

    # Map difficulty levels to a numeric scale
    difficulty_mapping = {difficulty: i for i, difficulty in enumerate(difficulty_order)}

    # Plot for each learner
    for learner_id in learner_ids:
        fig, axs = plt.subplots(len(difficulty_order), 2, figsize=(24, 14), gridspec_kw={'width_ratios': [3, 1]}, sharex='col')

        fig.suptitle(f'Progress and Attempted Difficulty Level of {learner_id} Over Iterations')

        # Loop through each difficulty and plot Q-values
        for idx, difficulty in enumerate(difficulty_order):
            ax1 = axs[idx, 0]
            # Apply some smoothing (because the Q-values can be noisy)
            values = np.convolve(learner_progress[learner_id][difficulty], np.ones(10)/10, mode='valid')
            ax1.plot(range(len(values)), values, label=difficulty)
            ax1.set_title(difficulty)
            ax1.label_outer()  # Only show outer labels
            ax1.set_ylim(0.0, 1.0)  # Set y-axis limits

        # Plot attempted difficulty level
        attempted_levels = [difficulty_mapping[level] for level in difficulty_levels_attempted[learner_id]]
        axs[0, 1].plot(range(len(attempted_levels)), attempted_levels, 'ko', markersize=2)
        axs[0, 1].set_yticks(range(len(difficulty_order)))
        axs[0, 1].set_yticklabels(difficulty_order)
        axs[0, 1].set_ylim(-1, len(difficulty_order))
        axs[0, 1].invert_yaxis()  # Invert to align with the left column
        axs[0, 1].set_title('Attempted Difficulty')

        # Remove the empty plots
        for idx in range(1, len(difficulty_order)):
            fig.delaxes(axs[idx, 1])

        # Set common labels
        plt.xlabel('Iterations')
        fig.text(0.04, 0.5, 'Smoothed Q-Values', va='center', rotation='vertical')
        fig.text(0.5, 0.04, 'Iterations', ha='center')

        # Adjust layout and show/save figure
        plt.tight_layout(rect=[0, 0.03, 1, 0.97])
        plt.savefig(f'results/{learner_id}_progress_strategy_{strategy}.png')  # Save the figure as a file
        plt.show()
