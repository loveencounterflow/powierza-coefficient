


'use strict'


############################################################################################################
CND                       = require 'cnd'
rpr                       = CND.rpr
badge                     = 'POWIERZA'
debug                     = CND.get_logger 'debug',     badge
alert                     = CND.get_logger 'alert',     badge
whisper                   = CND.get_logger 'whisper',   badge
warn                      = CND.get_logger 'warn',      badge
help                      = CND.get_logger 'help',      badge
urge                      = CND.get_logger 'urge',      badge
info                      = CND.get_logger 'info',      badge
echo                      = CND.echo.bind CND
POWIERZA                  = require '../../pkg'
{ to_width }              = require 'to-width'


#-----------------------------------------------------------------------------------------------------------
@_set_globals = ->
  _alert                    = alert
  globalThis.alert          = ( P... ) =>
    alert _border
    alert()
    _alert CND.reverse P...
    alert()
    alert _border
  globalThis.help           = help
  globalThis.urge           = urge
  globalThis.info           = info
  globalThis.debug          = debug
  return null

#-----------------------------------------------------------------------------------------------------------
sim = ( a, b ) -> Math.min ( POWIERZA.powierza a, b ), ( POWIERZA.osa a, b )

#-----------------------------------------------------------------------------------------------------------
@demo = ->
  @_set_globals()
  show = ( a, b ) ->
    pc  = POWIERZA.powierza a, b
    # pc /= Math.max ( Array.from b ).length, 0
    # pc /= Math.max ( Array.from a ).length, ( Array.from b ).length
    echo ( to_width ( rpr a ), 20 ), ( to_width ( rpr b ), 20 ), \
      ( CND.yellow to_width ( rpr pc ), 10 ), \
      ( CND.yellow to_width ( rpr POWIERZA.osa a, b ), 10 ), \
      ( CND.yellow to_width ( rpr sim a, b ), 10 )
    return null
  probes = [
    '',
    '(⿱宀珎貝)',
    '12345',
    '123456789',
    'a-b-c',
    'a-b-c-d-e-f',
    'a-bc',
    'a..........b....',
    'a..........b....c',
    'ab-c',
    'abc',
    'abc',
    'abcdef',
    '宀',
    '宀珎貝',
    '宀貝',
    '宀貝珎',
    ]
  pairs = ( ( [ a, b, ] for a in probes when a isnt b ) for b in probes ).flat()
  pairs.sort ( a, b ) ->
    # ( POWIERZA.powierza a... ) - ( POWIERZA.powierza b... )
    # ( POWIERZA.osa a... ) - ( POWIERZA.osa b... )
    ( sim a... ) - ( sim b... )
  show pair... for pair in pairs
  return null




############################################################################################################
if module is require.main then do =>
  @demo()
  return null



