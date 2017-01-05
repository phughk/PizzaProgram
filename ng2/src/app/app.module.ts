import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { HttpModule } from '@angular/http';

import { AppComponent } from './app.component';
import { PizzaComponent } from './pizza/pizza.component';
import { PizzaBaseComponent } from './pizza-base/pizza-base.component';
import { PizzaToppingComponent } from './pizza-topping/pizza-topping.component';

@NgModule({
  declarations: [
    AppComponent,
    PizzaComponent,
    PizzaBaseComponent,
    PizzaToppingComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
    HttpModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
