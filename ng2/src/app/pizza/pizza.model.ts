import { PizzaBaseComponent } from '../pizza-base/pizza-base.component';
import { PizzaToppingComponent } from '../pizza-topping/pizza-topping.component';

export class Pizza {
	base: PizzaBaseComponent;
	toppings: PizzaToppingComponent[];

	constructor(base: PizzaBaseComponent, toppings: PizzaToppingComponent[]) {
		this.base = base;
		this.toppings = toppings;
	}
}
