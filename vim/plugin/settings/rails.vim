let g:rails_projections = {
      \ "config/projections.json": {
      \   "command": "projections"
      \ },
      \ "spec/features/*_spec.rb": {
      \   "command": "feature",
      \   "template": "require 'spec_helper'\n\nfeature '%h' do\n\nend",
      \ },
      \ "spec/support/*.rb": {
      \   "command": "support"
      \ }}

let g:rails_gem_projections = {
      \ "active_model_serializers": {
      \   "app/serializers/*_serializer.rb": {
      \     "command": "serializer",
      \     "affinity": "model",
      \     "test": "spec/serializers/%s_spec.rb",
      \     "related": "app/models/%s.rb",
      \     "template": "class %SSerializer < ActiveModel::Serializer\nend"
      \   }
      \ },
      \ "draper": {
      \   "app/decorators/*_decorator.rb": {
      \     "command": "decorator",
      \     "affinity": "model",
      \     "test": "spec/decorators/%s_spec.rb",
      \     "related": "app/models/%s.rb",
      \     "template": "class %SDecorator < Draper::Decorator\nend"
      \   }
      \ },
      \ "factory_girl_rails": {
      \   "spec/factories.rb": {
      \     "command": "factories",
      \     "template": "FactoryGirl.define do\nend"
      \   }
      \ },
      \ "ember-rails": {
      \   "app/assets/javascripts/models/*.js.coffee": {
      \     "command": "jmodel",
      \     "alternate": "spec/javascripts/models/%s_spec.js.coffee",
      \     "template": "App.%S = DS.Model.extend"
      \   },
      \
      \   "app/assets/javascripts/controllers/*_controller.js.coffee": {
      \     "command": "jcontroller",
      \     "alternate": "spec/javascripts/controllers/%s_spec.js.coffee",
      \     "template": "App.%SController = Ember.ObjectController.extend"
      \   },
      \
      \   "app/assets/javascripts/views/*_view.js.coffee": {
      \     "command": "jview",
      \     "alternate": "spec/javascripts/views/%s_spec.js.coffee",
      \     "related": "app/assets/javascripts/templates/%s.hbs",
      \     "template": "App.%SView = Ember.View.extend"
      \   },
      \
      \   "app/assets/javascripts/router.js.coffee": {
      \     "command": "jrouter"
      \   },
      \
      \   "app/assets/javascripts/routes/*_route.js.coffee": {
      \     "command": "jroute",
      \     "alternate": "spec/javascripts/routes/%s_spec.js.coffee",
      \     "template": "App.%SRoute = Ember.Route.extend"
      \   },
      \
      \   "spec/javascripts/*_spec.js.coffee": {
      \     "command": "jspec",
      \     "alternate": "app/assets/javascripts/%s.js.coffee"
      \   },
      \
      \   "app/assets/javascripts/templates/*.hbs": {
      \     "command": "template",
      \     "alternate": "app/assets/javascripts/views/%s.js.coffee"
      \   }
      \ }}
