# Pizza Program
Newcastle University UK Computer Science coursework used to teach basics of OOP (in Java)
This github project contains the solution in C++ and Rust
(taken from Neil Spiers' CSC1021 module on programming at Newcastle University (UK) ).


----------------------------------------------------------------------------------------------------------------
# Task definition

Your task is to create a prototype which allows cost calculations for pizzas.
You will need at least three classes.

1) *Pizza*
This should carry all the essential information about a pizza, including a single base and some toppings.

2) *PizzaBase*
This class should model a pizza base. You will need at least two, one thick and one thin.

3) *PizzaTopping*
This class should model a pizza topping. You need at least four toppings, but you may provide as many as you choose.
If you are not an expert on pizza, you can find toppings by searching for a pizza menu on the web.

For later stages of this coursework, you will build a simple user interface. This is a prototype system,
so the user interface does NOT need to be complex; a simple command-line interface using Scanner and println
statements is entirely sufficient. However, as your classes may be used in several different situations, none of Pizza,
PizzaBase nor PizzaTopping should require any user interaction.

There are a number of features that you should attempt to achieve; please complete these in the order given.
    
    #######################
    #### Build a Pizza ####
    #######################
    
The initial version of your system should allow enable you to create objects for a pizza, one base and several toppings.
Each of the ingredients should have a cost associated with it. You should be able to combine these to create a pizza
which should have one and only one base, but many toppings. The finished pizza should calculate it’s cost.
You should write a simple class called PizzaBuild which contains a main method which demonstrates this working.

[N.B. Not using PizzaBuild class, using PizzaProgram for entry point]
    
    ###############################
    ### Create a User Interface ###
    ###############################
    
Create a new class called PizzaChoice. This should have a main method, and class should provide a simple user interface.
This should provide you with a choice of one base, and a number of toppings. It should compose the objects that you
created in the previous task. Once complete, you should print a short report showing all the ingredients, the price
for each and the total cost. The report should be formatted for easy reading. You may create additional classes,
beyond PizzaChoice should you find this useful.
    
    ###################
    ### Named Pizza ###
    ###################
    
Sometimes, there can be too many choices in life. Write a new class PizzaMenu, with a main method, which allows 
the user to choose from a menu of predefined pizza’s. For instance, a Margherita pizza consists of tomato, mozzarella 
and basil, on a thin base. You should provide the user with a choice of at least three pizzas.
As before, you should print a short report showing all the ingredients. You may create additional classes beyond PizzaMenu
should you find this useful.
    
    ##########################
    ### Vegetarian Options ###
    ##########################
    
Modify your PizzaToping, PizzaBase and Pizza classes so that the contain a property which describes whether they are
vegetarian or not.A pizza should be vegetarian if and only if all of its ingredients are. Extend the reports produced
by the last task so that they clearly state whether the pizza is suitable for vegetarians.
