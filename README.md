# tyr-rust
Moderation bot

## Developing on WSL
Create an ssh key pair for yourself to securely connect to your github
`ssh-keygen -t ed25519`

Click enter on all the prompts, then view the public key with
`cat ~/.ssh/id_ed25519.pub`

Copy the outputted line (it starts with ssh-ed25519) 
And paste it into your github ssh public keys at: https://github.com/settings/ssh/new

Inside wsl, make a working directory for your code

Install project dependencies
`sudo apt-get update && sudo apt-get upgrade && sudo apt-get install rustc git nano cargo screen`

Clone the repo
`mkdir ~/code/ && cd ~/code && git clone git@github.com:STEM-guild/tyr-rust.git && cd tyr-rust`

Configure the runtime environment
`cp .env.example .env && nano .env`

Press Ctrl + X when you're happy with the changes, then press Y twice

Launch the bot
`cargo run`

n.b. the first run will take longer due to compiling all the dependencies in the cargo file.
