import os

def prune_dataset(input_file, output_file):
    if not os.path.exists(input_file):
        print(f"Error: Input file '{input_file}' not found.")
        return

    with open(input_file, 'r', encoding='utf-8') as f_in:
        lines = f_in.readlines()
        
    unique_lines = sorted(list(set(lines)))
    
    with open(output_file, 'w', encoding='utf-8') as f_out:
        for line in unique_lines:
            if len(line.split()) >= 5:
                f_out.write(line)

    print(f"Dataset pruned successfully. Pruned data saved to '{output_file}'.")

if __name__ == "__main__":
    input_path = os.path.join("..", "data", "SLM-training-data.txt")
    output_path = os.path.join("..", "data", "SLM-training-data-pruned.txt")
    
    prune_dataset(input_path, output_path)
