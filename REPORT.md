# DS-210-Final-Project
Report 
The project works with a facebook dataset of friends. 
Link to data set: https://snap.stanford.edu/data/ego-Facebook.html 
The task was to find the two people with the most number of mutual friends. For the project, the data I received was already in the form of an undirected graph including the list of edges where each line had a pair of nodes ranging from 0 to 4031. My tasks included the following steps 
1. Reading the file in the form of a list of edges. 
2. Formulating subsequent vector of facebook users and a nested vector of friends for all the users (where a few lists were empty since the number was not present in the users list) 
3. Next step included converting each vector of friends to hashmaps to find their intersection in order to count the number of mutual friends and reporting the maximum number of mutual friends for each user separately and finding the user they have these mutual friends with. 
4. Out of the maximum number of mutual friends for each user we finally put them in a binary heap to print out the top five users that have the highest number of mutual friends, and the users they have those mutual friends with. 
The result for the project is of the form User X has Y mutual friends with User Z. 
