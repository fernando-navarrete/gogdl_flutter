// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'gogdl.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$GogError {
  String get message => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $GogErrorCopyWith<GogError> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GogErrorCopyWith<$Res> {
  factory $GogErrorCopyWith(GogError value, $Res Function(GogError) then) =
      _$GogErrorCopyWithImpl<$Res, GogError>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class _$GogErrorCopyWithImpl<$Res, $Val extends GogError>
    implements $GogErrorCopyWith<$Res> {
  _$GogErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _value.copyWith(
            message: null == message
                ? _value.message
                : message // ignore: cast_nullable_to_non_nullable
                      as String,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$GogError_AuthErrorImplCopyWith<$Res>
    implements $GogErrorCopyWith<$Res> {
  factory _$$GogError_AuthErrorImplCopyWith(
    _$GogError_AuthErrorImpl value,
    $Res Function(_$GogError_AuthErrorImpl) then,
  ) = __$$GogError_AuthErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$GogError_AuthErrorImplCopyWithImpl<$Res>
    extends _$GogErrorCopyWithImpl<$Res, _$GogError_AuthErrorImpl>
    implements _$$GogError_AuthErrorImplCopyWith<$Res> {
  __$$GogError_AuthErrorImplCopyWithImpl(
    _$GogError_AuthErrorImpl _value,
    $Res Function(_$GogError_AuthErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$GogError_AuthErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GogError_AuthErrorImpl extends GogError_AuthError {
  const _$GogError_AuthErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'GogError.authError(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GogError_AuthErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GogError_AuthErrorImplCopyWith<_$GogError_AuthErrorImpl> get copyWith =>
      __$$GogError_AuthErrorImplCopyWithImpl<_$GogError_AuthErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) {
    return authError(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) {
    return authError?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) {
    if (authError != null) {
      return authError(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) {
    return authError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) {
    return authError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) {
    if (authError != null) {
      return authError(this);
    }
    return orElse();
  }
}

abstract class GogError_AuthError extends GogError {
  const factory GogError_AuthError({required final String message}) =
      _$GogError_AuthErrorImpl;
  const GogError_AuthError._() : super._();

  @override
  String get message;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GogError_AuthErrorImplCopyWith<_$GogError_AuthErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GogError_GameErrorImplCopyWith<$Res>
    implements $GogErrorCopyWith<$Res> {
  factory _$$GogError_GameErrorImplCopyWith(
    _$GogError_GameErrorImpl value,
    $Res Function(_$GogError_GameErrorImpl) then,
  ) = __$$GogError_GameErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$GogError_GameErrorImplCopyWithImpl<$Res>
    extends _$GogErrorCopyWithImpl<$Res, _$GogError_GameErrorImpl>
    implements _$$GogError_GameErrorImplCopyWith<$Res> {
  __$$GogError_GameErrorImplCopyWithImpl(
    _$GogError_GameErrorImpl _value,
    $Res Function(_$GogError_GameErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$GogError_GameErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GogError_GameErrorImpl extends GogError_GameError {
  const _$GogError_GameErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'GogError.gameError(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GogError_GameErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GogError_GameErrorImplCopyWith<_$GogError_GameErrorImpl> get copyWith =>
      __$$GogError_GameErrorImplCopyWithImpl<_$GogError_GameErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) {
    return gameError(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) {
    return gameError?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) {
    if (gameError != null) {
      return gameError(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) {
    return gameError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) {
    return gameError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) {
    if (gameError != null) {
      return gameError(this);
    }
    return orElse();
  }
}

abstract class GogError_GameError extends GogError {
  const factory GogError_GameError({required final String message}) =
      _$GogError_GameErrorImpl;
  const GogError_GameError._() : super._();

  @override
  String get message;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GogError_GameErrorImplCopyWith<_$GogError_GameErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GogError_DepotErrorImplCopyWith<$Res>
    implements $GogErrorCopyWith<$Res> {
  factory _$$GogError_DepotErrorImplCopyWith(
    _$GogError_DepotErrorImpl value,
    $Res Function(_$GogError_DepotErrorImpl) then,
  ) = __$$GogError_DepotErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$GogError_DepotErrorImplCopyWithImpl<$Res>
    extends _$GogErrorCopyWithImpl<$Res, _$GogError_DepotErrorImpl>
    implements _$$GogError_DepotErrorImplCopyWith<$Res> {
  __$$GogError_DepotErrorImplCopyWithImpl(
    _$GogError_DepotErrorImpl _value,
    $Res Function(_$GogError_DepotErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$GogError_DepotErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GogError_DepotErrorImpl extends GogError_DepotError {
  const _$GogError_DepotErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'GogError.depotError(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GogError_DepotErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GogError_DepotErrorImplCopyWith<_$GogError_DepotErrorImpl> get copyWith =>
      __$$GogError_DepotErrorImplCopyWithImpl<_$GogError_DepotErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) {
    return depotError(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) {
    return depotError?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) {
    if (depotError != null) {
      return depotError(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) {
    return depotError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) {
    return depotError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) {
    if (depotError != null) {
      return depotError(this);
    }
    return orElse();
  }
}

abstract class GogError_DepotError extends GogError {
  const factory GogError_DepotError({required final String message}) =
      _$GogError_DepotErrorImpl;
  const GogError_DepotError._() : super._();

  @override
  String get message;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GogError_DepotErrorImplCopyWith<_$GogError_DepotErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GogError_SecureLinksErrorImplCopyWith<$Res>
    implements $GogErrorCopyWith<$Res> {
  factory _$$GogError_SecureLinksErrorImplCopyWith(
    _$GogError_SecureLinksErrorImpl value,
    $Res Function(_$GogError_SecureLinksErrorImpl) then,
  ) = __$$GogError_SecureLinksErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$GogError_SecureLinksErrorImplCopyWithImpl<$Res>
    extends _$GogErrorCopyWithImpl<$Res, _$GogError_SecureLinksErrorImpl>
    implements _$$GogError_SecureLinksErrorImplCopyWith<$Res> {
  __$$GogError_SecureLinksErrorImplCopyWithImpl(
    _$GogError_SecureLinksErrorImpl _value,
    $Res Function(_$GogError_SecureLinksErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$GogError_SecureLinksErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GogError_SecureLinksErrorImpl extends GogError_SecureLinksError {
  const _$GogError_SecureLinksErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'GogError.secureLinksError(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GogError_SecureLinksErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GogError_SecureLinksErrorImplCopyWith<_$GogError_SecureLinksErrorImpl>
  get copyWith =>
      __$$GogError_SecureLinksErrorImplCopyWithImpl<
        _$GogError_SecureLinksErrorImpl
      >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) {
    return secureLinksError(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) {
    return secureLinksError?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) {
    if (secureLinksError != null) {
      return secureLinksError(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) {
    return secureLinksError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) {
    return secureLinksError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) {
    if (secureLinksError != null) {
      return secureLinksError(this);
    }
    return orElse();
  }
}

abstract class GogError_SecureLinksError extends GogError {
  const factory GogError_SecureLinksError({required final String message}) =
      _$GogError_SecureLinksErrorImpl;
  const GogError_SecureLinksError._() : super._();

  @override
  String get message;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GogError_SecureLinksErrorImplCopyWith<_$GogError_SecureLinksErrorImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GogError_DownloadErrorImplCopyWith<$Res>
    implements $GogErrorCopyWith<$Res> {
  factory _$$GogError_DownloadErrorImplCopyWith(
    _$GogError_DownloadErrorImpl value,
    $Res Function(_$GogError_DownloadErrorImpl) then,
  ) = __$$GogError_DownloadErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$GogError_DownloadErrorImplCopyWithImpl<$Res>
    extends _$GogErrorCopyWithImpl<$Res, _$GogError_DownloadErrorImpl>
    implements _$$GogError_DownloadErrorImplCopyWith<$Res> {
  __$$GogError_DownloadErrorImplCopyWithImpl(
    _$GogError_DownloadErrorImpl _value,
    $Res Function(_$GogError_DownloadErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$GogError_DownloadErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GogError_DownloadErrorImpl extends GogError_DownloadError {
  const _$GogError_DownloadErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'GogError.downloadError(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GogError_DownloadErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GogError_DownloadErrorImplCopyWith<_$GogError_DownloadErrorImpl>
  get copyWith =>
      __$$GogError_DownloadErrorImplCopyWithImpl<_$GogError_DownloadErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) {
    return downloadError(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) {
    return downloadError?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) {
    if (downloadError != null) {
      return downloadError(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) {
    return downloadError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) {
    return downloadError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) {
    if (downloadError != null) {
      return downloadError(this);
    }
    return orElse();
  }
}

abstract class GogError_DownloadError extends GogError {
  const factory GogError_DownloadError({required final String message}) =
      _$GogError_DownloadErrorImpl;
  const GogError_DownloadError._() : super._();

  @override
  String get message;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GogError_DownloadErrorImplCopyWith<_$GogError_DownloadErrorImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GogError_SavesErrorImplCopyWith<$Res>
    implements $GogErrorCopyWith<$Res> {
  factory _$$GogError_SavesErrorImplCopyWith(
    _$GogError_SavesErrorImpl value,
    $Res Function(_$GogError_SavesErrorImpl) then,
  ) = __$$GogError_SavesErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$GogError_SavesErrorImplCopyWithImpl<$Res>
    extends _$GogErrorCopyWithImpl<$Res, _$GogError_SavesErrorImpl>
    implements _$$GogError_SavesErrorImplCopyWith<$Res> {
  __$$GogError_SavesErrorImplCopyWithImpl(
    _$GogError_SavesErrorImpl _value,
    $Res Function(_$GogError_SavesErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$GogError_SavesErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GogError_SavesErrorImpl extends GogError_SavesError {
  const _$GogError_SavesErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'GogError.savesError(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GogError_SavesErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GogError_SavesErrorImplCopyWith<_$GogError_SavesErrorImpl> get copyWith =>
      __$$GogError_SavesErrorImplCopyWithImpl<_$GogError_SavesErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) {
    return savesError(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) {
    return savesError?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) {
    if (savesError != null) {
      return savesError(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) {
    return savesError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) {
    return savesError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) {
    if (savesError != null) {
      return savesError(this);
    }
    return orElse();
  }
}

abstract class GogError_SavesError extends GogError {
  const factory GogError_SavesError({required final String message}) =
      _$GogError_SavesErrorImpl;
  const GogError_SavesError._() : super._();

  @override
  String get message;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GogError_SavesErrorImplCopyWith<_$GogError_SavesErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GogError_ProtonErrorImplCopyWith<$Res>
    implements $GogErrorCopyWith<$Res> {
  factory _$$GogError_ProtonErrorImplCopyWith(
    _$GogError_ProtonErrorImpl value,
    $Res Function(_$GogError_ProtonErrorImpl) then,
  ) = __$$GogError_ProtonErrorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$GogError_ProtonErrorImplCopyWithImpl<$Res>
    extends _$GogErrorCopyWithImpl<$Res, _$GogError_ProtonErrorImpl>
    implements _$$GogError_ProtonErrorImplCopyWith<$Res> {
  __$$GogError_ProtonErrorImplCopyWithImpl(
    _$GogError_ProtonErrorImpl _value,
    $Res Function(_$GogError_ProtonErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$GogError_ProtonErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GogError_ProtonErrorImpl extends GogError_ProtonError {
  const _$GogError_ProtonErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'GogError.protonError(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GogError_ProtonErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GogError_ProtonErrorImplCopyWith<_$GogError_ProtonErrorImpl>
  get copyWith =>
      __$$GogError_ProtonErrorImplCopyWithImpl<_$GogError_ProtonErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) authError,
    required TResult Function(String message) gameError,
    required TResult Function(String message) depotError,
    required TResult Function(String message) secureLinksError,
    required TResult Function(String message) downloadError,
    required TResult Function(String message) savesError,
    required TResult Function(String message) protonError,
  }) {
    return protonError(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? authError,
    TResult? Function(String message)? gameError,
    TResult? Function(String message)? depotError,
    TResult? Function(String message)? secureLinksError,
    TResult? Function(String message)? downloadError,
    TResult? Function(String message)? savesError,
    TResult? Function(String message)? protonError,
  }) {
    return protonError?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? authError,
    TResult Function(String message)? gameError,
    TResult Function(String message)? depotError,
    TResult Function(String message)? secureLinksError,
    TResult Function(String message)? downloadError,
    TResult Function(String message)? savesError,
    TResult Function(String message)? protonError,
    required TResult orElse(),
  }) {
    if (protonError != null) {
      return protonError(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GogError_AuthError value) authError,
    required TResult Function(GogError_GameError value) gameError,
    required TResult Function(GogError_DepotError value) depotError,
    required TResult Function(GogError_SecureLinksError value) secureLinksError,
    required TResult Function(GogError_DownloadError value) downloadError,
    required TResult Function(GogError_SavesError value) savesError,
    required TResult Function(GogError_ProtonError value) protonError,
  }) {
    return protonError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GogError_AuthError value)? authError,
    TResult? Function(GogError_GameError value)? gameError,
    TResult? Function(GogError_DepotError value)? depotError,
    TResult? Function(GogError_SecureLinksError value)? secureLinksError,
    TResult? Function(GogError_DownloadError value)? downloadError,
    TResult? Function(GogError_SavesError value)? savesError,
    TResult? Function(GogError_ProtonError value)? protonError,
  }) {
    return protonError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GogError_AuthError value)? authError,
    TResult Function(GogError_GameError value)? gameError,
    TResult Function(GogError_DepotError value)? depotError,
    TResult Function(GogError_SecureLinksError value)? secureLinksError,
    TResult Function(GogError_DownloadError value)? downloadError,
    TResult Function(GogError_SavesError value)? savesError,
    TResult Function(GogError_ProtonError value)? protonError,
    required TResult orElse(),
  }) {
    if (protonError != null) {
      return protonError(this);
    }
    return orElse();
  }
}

abstract class GogError_ProtonError extends GogError {
  const factory GogError_ProtonError({required final String message}) =
      _$GogError_ProtonErrorImpl;
  const GogError_ProtonError._() : super._();

  @override
  String get message;

  /// Create a copy of GogError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GogError_ProtonErrorImplCopyWith<_$GogError_ProtonErrorImpl>
  get copyWith => throw _privateConstructorUsedError;
}
