import os
import glob

filenames = glob.glob("./media/*")
print(filenames)

for i in range(len(filenames)):
    name = str(filenames[i].split(".")[:-1])
    extension = filenames[i].split(".")[-1]
    # os.rename(f"{i}.{extension}", filenames[i])
    os.system(f"cp ./media/{filenames[i]} ./media/{name}")

