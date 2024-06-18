import os



def rename_directories(base_dir):
    # Iterate over all items in the base directory
    for item in os.listdir(base_dir):
        item_path = os.path.join(base_dir, item)
        
        print(item_path)
        # Check if the item is a directory and starts with '.'
        if os.path.isdir(item_path) and item.startswith('.'):
            new_item_name = item[1:]
            print('hello', new_item_name)
            new_item_path = os.path.join(base_dir, new_item_name)
            
            try:
                os.rename(item_path, new_item_path)
                print(f"Renamed {item_path} to {new_item_path}")
            except Exception as e:
                print(f"Error renaming directory {item_path}: {e}")

# Specify the base directory
base_dir = '.'

# Call the function to rename directories
rename_directories(base_dir)
