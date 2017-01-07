import { Component, OnInit } from '@angular/core';

import { PizzaBase } from '../pizza-base.model';
import { PizzaTopping } from '../pizza-topping.model';

@Component({
  selector: 'app-pizza',
  templateUrl: './pizza.component.html',
  styleUrls: ['./pizza.component.css']
})
export class PizzaComponent implements OnInit {

	toppings: PizzaTopping[];
	base: PizzaBase;
	name: string;

  constructor() { 
		this.name = "Hardcoded name";
		this.toppings = [
			new PizzaTopping("Salami"),
			new PizzaTopping("Cheese")
		]
		this.base = new PizzaBase();
	}

  ngOnInit() {
  }

}
