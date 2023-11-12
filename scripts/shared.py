import os
import shutil

def copy_and_archive_image(src='./image.png', archive_dir='./archive'):
    if not os.path.exists(archive_dir):
        os.makedirs(archive_dir)

    existing_files = os.listdir(archive_dir)
    max_num = -1
    for file_name in existing_files:
        if file_name.startswith("image-") and file_name.endswith(".png"):
            try:
                num = int(file_name[6:-4])
                max_num = max(max_num, num)
            except ValueError:
                continue
    
    next_num = max_num + 1
    new_file_path = os.path.join(archive_dir, f"image-{next_num}.png")
    shutil.copy(src, new_file_path)
    print(f"Copied '{src}' to '{new_file_path}'")
