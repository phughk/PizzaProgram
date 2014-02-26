// PizzaProgram.cpp : Defines the entry point for the console application.
//

#include "stdafx.h"
#include "Pizza.h"

using namespace std;

PizzaTopping* generatePizzaToppingFromMenu()
{
	PizzaTopping* topping = new PizzaTopping;
	char name[20];
	cout << "Enter topping name: ";
	//cin >> name; Won't read empty lines
	cin.getline(name, 20);

	if (strcmp(name, "")==0)
	{
		cout << "Exiting\n";
		return NULL;
		delete topping;
	}
	else
	{
		cout << "Adding new topping: " << name << "\n";
		topping->setName(name);
		return topping;
	}
}

PizzaBase* generatePizzaBaseFromMenu()
{
	PizzaBase* base  = new PizzaBase;
	char buff[40];
//	float v;

	cout << "Enter pizza base name: ";
	cin.getline(buff, 40);
//	cin >> buff;
	base->setName(buff);

	cout << "Enter pizza base cost: ";
	cin.getline(buff, 40);
//	cin >> v;
	base->setCost(atof(buff));

	return base;
}

Pizza* generatePizzaFromMenu()
{
	Pizza* p = new Pizza;
	PizzaBase* base=0;
	PizzaTopping* top=0;

	//Setup Pizza
	char buff[20];
	cout << "[Builder] Enter pizza name: ";
	cin.getline(buff, 20);

	p->setName(buff);

	base=generatePizzaBaseFromMenu();
	cout << "[Builder] Generated pizza base: " << *base << "\n";
	p->setBase(base);

	while ((top=generatePizzaToppingFromMenu())!=NULL)
	{
		cout << "[Builder]  Adding topping: " << *top << "\n";
		p->addTopping(top);
	}
	cout << "[Builder] Finished pizza build: " << *p << "\n";
	return p;
}

#ifdef _WIN32
int _tmain(int argc, _TCHAR* argv[])
#elif __unix__
int main(int argc, char** argv)
#endif
{
	Pizza* pizza=generatePizzaFromMenu();
	cout << "End builder pizza: " << *pizza << "\n";
	cout << "Receipt:\n";
	pizza->getReceipt(cout);
#ifdef _WIN32
	system("pause");
#endif
}

