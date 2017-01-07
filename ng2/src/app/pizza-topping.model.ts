export class PizzaTopping {

	name: string;
	price: number;
	vegeterian: boolean = false;

  constructor(name: string,
							price: number = 1.0,
							vegeterian: boolean = false) {
		this.name = name;
		this.price = price;
		this.vegeterian = vegeterian;
	}
}
