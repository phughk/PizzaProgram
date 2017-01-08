import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { HttpModule } from '@angular/http';
import { RouterModule, Routes } from '@angular/router';

import { AppComponent } from './app.component';
import { PizzaComponent } from './pizza/pizza.component';
import { PizzaMenuComponent } from './pizza-menu/pizza-menu.component';
import { CreatePizzaComponent } from './create-pizza/create-pizza.component';

const routes: Routes = [
	{ path: '', redirectTo: 'menu', pathMatch: 'full' },
	{ path: 'menu', component: PizzaMenuComponent },
	{ path: 'pizza', component: PizzaComponent },
	{ path: 'create', component: CreatePizzaComponent },
];

@NgModule({
  declarations: [
    AppComponent,
    PizzaComponent,
    PizzaMenuComponent,
    CreatePizzaComponent
  ],
  imports: [
    BrowserModule,
		RouterModule.forRoot(routes),
    FormsModule,
    HttpModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
