#include "stdafx.h"
#include "Pizza.h"


Pizza::Pizza(void)
{
	setBase(NULL);
	//toppings=(PizzaTopping*) malloc(sizeof(PizzaTopping)*MAX_TOPPINGS);
//	for (int i=0; i<MAX_TOPPINGS; i++)
//	{
//		toppings = NULL;
//	}

	//toppings = new std::vector<PizzaTopping>; // Not needed since it is not a pointer
}


Pizza::~Pizza()
{
	//delete base;
}

void Pizza::setBase(PizzaBase* b)
{
	base=b;
}

char* Pizza::getName() 
{
	return name;
}

void Pizza::setName(char* c)
{
	strcpy(name, c);
}

float Pizza::getCost() const
{
	float total=0.0;
	total+=base->getCost();

	for (unsigned int i=0; i<toppings.size(); i++)
	{
		total+=toppings[i]->getCost();
	}	

	return total;
}

void Pizza::addTopping(PizzaTopping* top)
{
	toppings.push_back(top);
}

void Pizza::removeToppingByIndex(int i)
{
	if (toppingIndexOutOfRange(i)) {} // TODO Throw exception
	else toppings.erase(toppings.begin()+i);
}

int Pizza::getToppingCount() const
{
	return toppings.size();
}

const PizzaTopping* Pizza::getToppingByIndex(int i)
{
	if (toppingIndexOutOfRange(i)) return NULL; // TODO Throw exception
	return toppings[i];
}

const unsigned char Pizza::toppingIndexOutOfRange(int i)
{
	if ((i<0) | (i>getToppingCount())) return 1;
	return 0;
}

const void Pizza::getReceipt(std::ostream& strm)
{
	//throw std::logic_error("The method or operation is not implemented.");
	strm << "\nPizza: \n";
	strm << "\tname: " << getName() << "\n";
	strm << "\tcost: " << getCost() << "\n";
	strm << "\tbase: " << *base << "\n";
	strm << "\ttoppings: " << "\n";
	for (int i=0; i<toppings.size(); i++)
	{
		strm << "\t\t" << *toppings[i] << "\n";
	}
}

std::ostream& operator<<(std::ostream& strm, const Pizza p)
{
	char buff[40];
	const char* n = p.getBase()->getName();
	int c = p.getToppingCount();
	sprintf(&(buff[0]), "Pizza (%s, %d toppings)", n, c);
	strm << buff;
	return strm;
}
