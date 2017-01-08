import { Injectable } from '@angular/core';

@Injectable()
export class MenuService {

	values: string[];

  constructor() {
		this.values = []
	}

	click() {
		let val = this.values.length;
		this.values.push(""+val);
		console.log(`Added ${val}`);
	}

}
