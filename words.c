/* WORDS - a program that reads one or more files, sorts the words in the files, and 
 * then prints the word prefixed by number of times it appears in the file
 *
 * Usage - sort words from input file(s) in order
 *
 * Inputs - file names
 * Outputs - words printed in order, preceded by the number of occurrences of that word in the file(s)
 *
 * Exit code - EXIT_SUCCESS (0) upon successful compilation
 *					 EXIT_FAILURE (1)  if: file cannot be read
 *					 								  ran out of memory for input file(s)
 *
 * Written for ECS 030, Fall 2015
 * Bryan Zhao, Nov. 27, 2015
 * Original program written
 * December 4, 2015
 * Modification for multiple files added
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h> 
#include <malloc/malloc.h>!

/* structure for linked list */

struct num {
	 char word[99]; /* field with word to be sorted */
	 int count; /* count (number of times a word appears in file) */
	 struct num *next; /* points to next node or word in list */
};
	 
/*
 * pointer to the first element (the head) of the list
 * NULL if there's nothing in the list
 */
struct num *head = NULL;

/*
 * create a new node
 * and initialize the two fields
 */
struct num *createnode(char input_word[99])
{
	struct num *p;		/* pointer to new space */
	
	/* create the element, reporting errors */
	if ((p = (struct num *) malloc(sizeof(struct num))) == NULL)
	return(NULL);
	
	/* initialize the element */
	strcpy(p->word,input_word);
	p->next = NULL;
	p->count = 1;

	/* return a pointer to the new entity */
	return(p);
}

/*
 * insert the element that new points to into the linked list,
 * and return a pointer to the (possibly new) head of the list
 */
struct num *insert(struct num *new)
{
	struct num *prev, *temp; /* references to neighboring nodes */
	
	/* empty list: put head at the front */
	if (head == NULL)
		return(new);

	/* it goes before the first element */
	if (strcmp(head->word, new->word) > 0){ /* compare ASCII codes between fields to sort in order */
		new->next = head;
		return(new);
	}

	/*
	 * now walk the list
	 * from here on in, prev->next == temp
	 * we'll insert after prev and before temp
	 */
	prev = head;
	temp = head->next;
	while((temp != NULL) && (strcmp(temp->word, new->word) < 0)){
		/* advance prev and temp */
		prev = temp;
		temp = temp->next;
	}

	/*
	 * here's the insertion
	 * make prev->next the new element
	 * and new->next the one temp points to
	 * as long as there are no repetitions with previous words
	 */
	
		new->next = temp;
		prev->next = new;

	
	if (strcmp(temp->word, new->word) == 0) /* increment field count if the word is repeated in any other node */
		temp->count = temp->count + 1;

	
		
	
		

	/* return the pointer to the head of the list */
	return(head);
}

/*
 * the main routine
 * read in words and sort them
 */
 
 int main(int argc, char *argv[]){
	 /* insert main code here */
 
 	char c; /* character to be read from input by fgetc */
	struct num *p;	/* pointer to element for linked list */
	char current_word[99]; /* buffer for each input word from file */
	FILE *fp; /* file pointer to input files to be read */
	int lineno = 0; /* current line number being read in input */
	int i = 0; /* increment of letter in single word */
	int j; /* for loop variable to open files */

	memset(current_word, '\0', sizeof(current_word)); /* clear space in the array current_word for next incoming word in buffer */
	
	 /*if file cannot be read, print error message
	  *open multiple files!
	  */
	if (argc >= 2){
		 for (j = 1; j <= (argc - 1); j++){
		 
			 if ((fp = fopen(argv[j], "r")) == NULL)  /* error if file cannot be read */
				perror(argv[j]);
 
			 while ((c = fgetc(fp)) != EOF){
				if (	isalnum(c)){ /* form words with only alphanumeric characters */
					current_word[i] = c;
					i++; /* increment index of letter in current word */	
				}
		
				if (c == '\n'){
					++lineno; /* increment current line number being read */
				}
		
				/* insert new element into linked list once the end of the word has been reached or a newline character has been encountered*/
				if (isspace(c) || c == '\n'){
					head = insert(p);
					if ((p = createnode(current_word)) == NULL){
						fprintf(stderr, "%s, file %s, line %d: ran out of memory\n", current_word, argv[j], lineno);
						exit(1);
					}
					i = 0; /* restart counter for next word */
					memset(current_word, '\0', sizeof(current_word)); /* clear space in the array current_word for next incoming word in buffer */
				}
			}
		}
	}
	
	/*
	 * print the list
	 * start at head, print character field and count field of each element 
	 * and then go on to the next element
	 */
	for(p = head; p != NULL; p = p->next)
		printf("%5d %s\n", p->count, p->word); 
		
	fclose(fp);
	 /* bye-bye! */
	 exit(0);
 }
