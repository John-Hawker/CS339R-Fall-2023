use the following in the command line to use an authentication token from git hub to allow push and pull from the repository.
git clone https://<token_here>@github.com/<user_name>/<repo_name>.git

To get a token:
1. go to github.com 
2. login to your account
3. click on your user profile (top right corner image)
4. in the drop down menu select "Settings"
5. on the left hand side of the screen scroll down to and select "Developer settings"
6. on the left hand side of the screen select "Personal access tokens" >  select "Tokens (classic)"
7. on th top right hand side of the screen select the drop down menu "Generate new token" > select "Generate new token (classic)"
8. add a note to the token
9. select an expiration time for this token
10. to have full scope of authority on the repository from your local machine select all of the checkboxes
11. select the green "Generate token" icon at the botom of the screen
12. copy the newly added token
13. using the above command line instruction paste the new token in to the <token_here> portion of the line
14. put in your user_name and repo_name into the above command line instruction
    example: git clone https://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@github.com/John-Hawker/CS339R-Fall-2023.git
15. copy the instruction, open the terminal, put the copied instruction in the terminal command line, and press enter

To check the status of your local copy of your github repository
1. open a terminal and change your working directory to your local copy of your repository
2. use the command "git status" to see what you need to do

To stage a commit from your local repository
1. open a terminal and change your working directory to your local copy of your repository
2. check the status of your local copy to see if you need to make a commit
3. if you need to make a commit use the command 'git commit -m "<add a message about the commit here>"'

To push your local changes to your git hub repository
1. open a terminal and change your working directory to your local copy of your repository
2. check the status of your local copy to see if you are staged to make a push to your git hub repository
3. if you are ready to push your changes use the command 'git push origin main' if you are working on the main branch

To update your local repository from git hub
1. open a terminal and change your working directory to your local copy of your repository
2. use the command 'git pull' to get an update from your git hub repository to your local repository