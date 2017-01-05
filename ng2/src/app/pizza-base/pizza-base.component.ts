import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-pizza-base',
  templateUrl: './pizza-base.component.html',
  styleUrls: ['./pizza-base.component.css']
})
export class PizzaBaseComponent implements OnInit {

	name: string;

  constructor() { 
		this.name = "Example Pizza Base Name";
	}

  ngOnInit() {
  }

}
