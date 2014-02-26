// PizzaProgram.cpp : Defines the entry point for the console application.
//

#include "stdafx.h"
#include "Pizza.h"

using namespace std;

PizzaTopping* generatePizzaToppingFromMenu()
{
	PizzaTopping* topping = new PizzaTopping;
	char buff[20];
	cout << "Enter topping name: ";
	//cin >> name; Won't read empty lines
	cin.getline(buff, 20);

	if (strcmp(buff, "")==0)
	{
		cout << "Exiting\n";
		return NULL;
		delete topping;
	}
	else
	{
		topping->setName(buff);
		cout << "Enter topping price: ";
		cin.getline(buff, 20);
		topping->setCost(atoi(buff));
		cout << "Adding new topping: " << topping->getName() << "\n";
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

int testMain()
{
	Pizza* pizza=generatePizzaFromMenu();
	cout << "End builder pizza: " << *pizza << "\n";
	cout << "Receipt:\n";
	pizza->getReceipt(cout);
#ifdef _WIN32
	system("pause");
#endif
	return 0;
}

void mainProgram()
{
	vector<Pizza*> pizzas;
	vector<PizzaTopping*> topps;
	vector<PizzaBase*> bases;
	char buff[30];
	while (1)
	{
		cout << "\n" << "1. Make pizza base\n";
		cout << "2. Make pizza topping\n";
		cout << "3. Make pizza\n";
		cout << "4. Make order\n";
		cout << "5. List bases\n";
		cout << "6. List toppings\n";
		cout << "Enter choice: ";
		cin.getline(buff, 30);
		cout << "\n";
		if (strcmp(buff, "")==0)
		{
			cout << "Exit? ";
			cin.getline(buff, 30);
			if (strcmp(buff, "")!=0) break;
		}
		else if (strcmp(buff, "1")==0)
		{
			PizzaBase* base = generatePizzaBaseFromMenu();
			bases.push_back(base);
		}
		else if (strcmp(buff, "2")==0)
		{
			PizzaTopping* topping = generatePizzaToppingFromMenu();
			topps.push_back(topping);
		}
		else if (strcmp(buff, "3")==0)
		{
			cout << "Unimplemented\n";
		}
		else if (strcmp(buff, "4")==0)
		{
			cout << "Unimplemented\n";
		}
		else if (strcmp(buff, "5")==0)
		{
			// List bases
			for (unsigned int i=0; i<bases.size(); i++)
			{
				cout << *bases[i] << "\n";
			}
		}
		else if (strcmp(buff, "6")==0)
		{
			for (unsigned int i=0; i<topps.size(); i++)
			{
				cout << *topps[i] << "\n";
			}
		}
		else
		{
			cout << "Invalid option\n";
		}
	}
}

#ifdef _WIN32
int _tmain(int argc, _TCHAR* argv[])
#elif __unix__
int main(int argc, char** argv)
#endif
{
	//return testMain();
	mainProgram();
	return 0;
}


