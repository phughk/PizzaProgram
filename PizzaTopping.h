#pragma once

class PizzaTopping
{
public:
	PizzaTopping(void);
	~PizzaTopping(void);
};
std::ostream& operator<< (std::ostream &strm, const PizzaTopping& p);

