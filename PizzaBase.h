#pragma once

#define PIZZA_BASE_NAME_SIZE 40

class PizzaBase
{
private:
	char name[PIZZA_BASE_NAME_SIZE];
	float cost;
	friend std::ostream& operator<< (std::ostream& strm, const PizzaBase b);

public:
	PizzaBase(void);
	~PizzaBase(void);

	// Getters/setters
	void setName(char*);
	const char* getName() {return name;};

	void setCost(float);
	const float getCost() {return cost;};

};

