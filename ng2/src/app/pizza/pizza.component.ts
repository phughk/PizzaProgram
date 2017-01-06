import { Component, OnInit } from '@angular/core';

import { PizzaBaseComponent } from '../pizza-base/pizza-base.component';
import { PizzaToppingComponent } from '../pizza-topping/pizza-topping.component';

@Component({
  selector: 'app-pizza',
  templateUrl: './pizza.component.html',
  styleUrls: ['./pizza.component.css']
})
export class PizzaComponent implements OnInit {

	toppings: string[];
	base: PizzaBaseComponent;

  constructor() { 
		this.toppings = ["Salami", "Pepperoni"]
		//this.base = {name: "thin", price=4}
		this.base = new PizzaBaseComponent();
	}

  ngOnInit() {
  }

}
