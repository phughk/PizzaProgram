import { Component, OnInit, Input} from '@angular/core';

@Component({
  selector: 'app-pizza-topping',
  templateUrl: './pizza-topping.component.html',
  styleUrls: ['./pizza-topping.component.css']
})
export class PizzaToppingComponent implements OnInit {

	@Input() name: string;

  constructor() { }

  ngOnInit() {
  }

}
