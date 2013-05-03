autocmd User Rails Rnavcommand steps       features/step_definitions          -glob=**/*
autocmd User Rails Rnavcommand routes      config                             -glob=*.*  -default=routes.rb -suffix=
autocmd User Rails Rnavcommand initializer config/initializers                -glob=**/*
autocmd User Rails Rnavcommand config      config                             -glob=**/*
autocmd User Rails Rnavcommand factories   spec test                          -glob=**/* -default=factories
autocmd User Rails Rnavcommand job         app/jobs                           -glob=**/*                    -suffix=.rb
autocmd User Rails Rnavcommand serializer  app/serializers                    -glob=**/*                    -suffix=.rb
autocmd User Rails Rnavcommand decorator   app/decorators                     -glob=**/*                    -suffix=.rb
autocmd User Rails Rnavcommand template    app/assets/templates               -glob=**/*                    -suffix=.jst.ejs
autocmd User Rails Rnavcommand jmodel      app/assets/javascripts/models      -glob=**/*                    -suffix=
autocmd User Rails Rnavcommand jview       app/assets/javascripts/views       -glob=**/*                    -suffix=
autocmd User Rails Rnavcommand jcollection app/assets/javascripts/collections -glob=**/*                    -suffix=
autocmd User Rails Rnavcommand jrouter     app/assets/javascripts/routers     -glob=**/*                    -suffix=
