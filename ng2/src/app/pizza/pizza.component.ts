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

  constructor() { 
		this.toppings = ["Salami", "Pepperoni"]
	}

  ngOnInit() {
  }

}
