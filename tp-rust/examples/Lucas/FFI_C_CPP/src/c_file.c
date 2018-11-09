#include <stdio.h>
#include <stdlib.h>

typedef struct{
   int width;
   int height;
}rectangle_t;

int double_input_c(int input){
	printf("%s\n", "Este mensaje se imprimio en C! :D");
	return input * 2;
}

void int_pointer_c(int * a){
	printf("El dato pasado por puntero es: %i\n", *a);
	(*a)++;
}

int rectangle_area_c(rectangle_t * rect){
	return (rect->width)*(rect->height);
}