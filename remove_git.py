import os
import shutil

def remove_git_directories(root_dir):
    """
    Recursively find and remove .git directories in subfolders of the given root directory.
    """
    for dirpath, dirnames, filenames in os.walk(root_dir):
        # Check if .git exists in the current directory
        if ".git" in dirnames:
            git_path = os.path.join(dirpath, ".git")
            print(f"Removing: {git_path}")
            shutil.rmtree(git_path)
            # Remove the directory from the list to prevent os.walk from processing it
            dirnames.remove(".git")
    
    print("Completed removing all .git directories.")

if __name__ == "__main__":
    # Set the path to your folder containing multiple Cargo projects
    cargo_projects_folder = os.getcwd()
    
    if os.path.exists(cargo_projects_folder):
        for folder in os.listdir():
            remove_git_directories(folder)
    else:
        print("Invalid path. Please enter a valid directory path.")
