#include "stdafx.h"
#include "PizzaTopping.h"


PizzaTopping::PizzaTopping(void)
{
}


PizzaTopping::~PizzaTopping(void)
{
}

std::ostream& operator<< (std::ostream &strm, const PizzaTopping& p)
{
	char buff[40];
	sprintf(&(buff[0]), "PizzaTopping");
	return strm << buff;
}
