# MinecraftStarter
This is a small tool that makes the Twitch Launcher go to the minecraft tab on startup.

# Tutorial
1. Download & Extract the MinecraftStarter.zip file from the release tab (Or compile the exe yourself if you want to)
2. Copy the MinecraftStarter.exe file into your Twitch Binary Path (Default is C:\Users\USERNAME\AppData\Roaming\Twitch\Bin, it might also be C:\Users\USERNAME\AppData\Roaming\Curse Client\Bin)
3. "Run" the Enable_MinecraftStarter.reg file.
4. Done

To undo the whole thing just run the Disable_MinecraftStarter.reg file and remove the MinecraftStarter.exe file from your twitch folder.

After an Twitch Update you might have to redo Step 2 because Twitch deleted the MinecraftStarter.exe

It doesn't appear to cause any issues but obviously don't report issues to Twitch you can't reproduce without this.

# How it works
The "home page" of the launcher is a simple command line argument for the TwitchUI. The registry file causes the MinecraftStarter.exe to run instead of the actual twitch ui. 
The MinecraftStarter.exe then changes the home page in the command line arguments and runs the actual TwitchUI. (It also copies the TwitchUI.exe because it wouldn't run with DEBUG_PROCESS)
