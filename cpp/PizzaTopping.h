#pragma once

class PizzaTopping
{
private:
	char name[20];
	float cost;

public:
	PizzaTopping(void);
	~PizzaTopping(void);
	
	const char* getName() const;
	void setName(char* c);

	float getCost() const;
	void setCost(float f);
};
std::ostream& operator<< (std::ostream &strm, const PizzaTopping& p);

