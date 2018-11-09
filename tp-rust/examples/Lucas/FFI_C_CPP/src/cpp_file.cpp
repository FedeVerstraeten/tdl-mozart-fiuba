#include <iostream>

extern "C"

int double_input_cpp(int input){
	std::cout<<"Este mensaje se imprimio en C++! :D"<<std::endl;
	return input * 2;
}