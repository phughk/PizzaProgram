import { Injectable } from '@angular/core';
import { PizzaComponent } from './pizza/pizza.component';

@Injectable()
export class MenuService {

	values: string[];
	pizzas: PizzaComponent[]

  constructor() {
		this.values = []
	}

	click() {
		let val = this.values.length;
		this.values.push(""+val);
		console.log(`Added ${val}`);
	}

}
