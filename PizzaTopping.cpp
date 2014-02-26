#include "stdafx.h"
#include "PizzaTopping.h"


PizzaTopping::PizzaTopping(void)
{
}


PizzaTopping::~PizzaTopping(void)
{
}

const char* PizzaTopping::getName() const
{
	return &(name[0]);
}

void PizzaTopping::setName(char* c)
{
	strcpy(name, c);
}

float PizzaTopping::getCost() const
{
	return cost;
}

void PizzaTopping::setCost(float f)
{
	cost=f;
}

std::ostream& operator<< (std::ostream &strm, const PizzaTopping& p)
{
	char buff[40];
	sprintf(&(buff[0]), "PizzaTopping (%s, %f)", p.getName(), p.getCost());
	return strm << buff;
}
