#pragma once

#include "PizzaBase.h"
#include "PizzaTopping.h"
#include <vector>

#define MAX_TOPPINGS 10
#define NAME_SIZE 20

class Pizza
{
private:
	char name[20];
	PizzaBase* base;
	std::vector<PizzaTopping*> toppings;
	friend std::ostream& operator<< (std::ostream& strm, const Pizza p);
	const unsigned char toppingIndexOutOfRange(int i);

public:
	Pizza(void);
	~Pizza(void);

	void setBase(PizzaBase*);
	PizzaBase* getBase() const {return base;};

	char* getName();
	void setName(char* c);
	float getCost() const;

	void addTopping(PizzaTopping*);
	void removeToppingByIndex(int);
	int getToppingCount() const;
	const PizzaTopping* getToppingByIndex(int);
	const void getReceipt(std::ostream& strm);
};

