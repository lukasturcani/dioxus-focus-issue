This repo shows an issue with dioxus ``prevent_default``. In the
HTML example ``working.html`` you can see that if the form has
focus and the button has uses ``event.preventDefault()``.
In the dioxus example, which should behave the same way, use of
``prvent_default`` does not stop the button from stealing focus
from the input element.

You can run the example with

.. code-block:: bash

  dx serve --hot-reload --open --port 8080
