#include "stdafx.h"
#include "PizzaBase.h"


PizzaBase::PizzaBase(void)
{
}


PizzaBase::~PizzaBase(void)
{
}

void PizzaBase::setName(char* n)
{
	strcpy(&(name[0]), n);
//	name=*n;
}

void PizzaBase::setCost(float c)
{
	cost=c;
}

std::ostream& operator<<(std::ostream& strm, const PizzaBase b)
{
	char buff[40];
	sprintf(&(buff[0]), "PizzaBase name=%s cost=%d", b.name, b.cost);
	return strm << buff;
}
