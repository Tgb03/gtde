
# **Use GIT with this project in case something goes wrong**

# GTDE: GTFO Datablock Editor

This project aims to make working with GTFO datablock projects easier. It mainly is a 
command line application that allows you to do various things ranging from creating projects
to adding specific datablock entries to your project or searching for entries.

# How to install:

### _Quick and easy way_

Go to **Releases** and download "gtde-installer.exe". Simply run it and the app will be installed
in the default folder.

### _Manual way_ (If you know what you are doing)

Go to **Releases** and download "gtde.exe". Place this file wherever you want and then modify the
environment variable `PATH` to link to the folder with the "gtde.exe" file. You can also drop this directly in the folder of the project where you are using it but you will not be able to use it anywhere else without duplicating the executable.

### _Linux way_ (I'm sorry)

You have to download "gtde" and put it in the `/usr/local/bin` folder.

# How to use:

Opening a cmd or powershell allows you to use the `gtde` command. If you type `gtde -h` it will list all
existing commands. Use that to understand what the app can do. 

### <ins>How to make a new project</ins>

There are 2 commands that allow you to make a new project. 

- `gtde new <PROJECT_NAME>` - creates a new folder with project name and then runs `gtde init` inside it. 
- `gtde init` - creates all the project files and setup inside this folder.

If your project has some other setup than this I really recommend using this one as I guarantee it works with thunderstore publishing system and all the modmanagers: Thunderstore, R2Modman, Gale. I also strongly recommend keeping your modded profile separate from the project folder. For this specific case `gtde build` command exists. I will talk about it later.

### <ins>Building the project</ins>

`gtde init` has also created an important file named `gtde.config`. Modify the `profile_path` entry to link
to a specific modding profile. This will be used to export your project there so you can quickly test it. 

Now you can build the project using `gtde build` which will take the current files and export them into the profile at `profile_path`. MTFO will load them or even automatically reload the files if the game is open and MTFO config is set to auto reload. 

If you have a build that is ready to be published simply run `gtde build -r` which will create inside the `output` folder 2 files: the unzipped project and the zipped project. The .zip file is already ready to be published to thunderstore. If at any point anything goes wrong, make sure to read the output of `gtde`, it will tell you what happened, mostly. If not, please let me know.

### <ins>Grab Datablock</ins>

This is a quality of life. If you type `gtde grab-db <DATABLOCK>` it will copy and paste into your project the default vanilla datablock. Beware this does not work if you already have an existing one as a failsafe. If it already exists you need to delete the existing one.

### <ins>Create functions</ins>

Now that you have working project, here's the most powerful feature `create`, beware this feature is still somewhat limited right now but I will develop it over time.

`gtde create -h` allows you to do specific stuff like adding entries to specific datablocks or even multiple datablocks. How it works: When you run it for the first time it will create some files in the `gtde-create` folder. You will then edit these files and then, when you run `gtde create <FILE_USED>`, gtde will read said file and try to add that entry to your project. It will then tell you which `persistentID` it got and can be later used by you. 

